#[cfg(test)]
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
use zb_aps::apsde::DataIndication;
use zb_core::{Cluster, Direction};
use zb_zcl::{Cluster as ZclCluster, Frame, Scope};

use crate::MPSC_CHANNEL_SIZE;

/// A received ZCL frame delivered to an internal subscriber.
#[derive(Clone, Debug)]
pub struct Received {
    /// APSDE indication containing the parsed ZCL frame and all receive metadata.
    pub indication: DataIndication<Frame<ZclCluster>, (), ()>,
}

/// Header fields selecting ZCL frames for an internal subscriber.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Filter {
    cluster: Cluster,
    scope: Scope,
    direction: Direction,
}

/// Weak delivery handle for one filtered ZCL subscription.
#[derive(Clone, Debug)]
pub struct Subscription {
    filter: Filter,
    messages: WeakSender<Received>,
}

/// Receiving half of an internal ZCL subscription.
///
/// The strong sender is retained here so the ZCL actor can hold only a weak sender. Dropping this
/// receiver therefore breaks the subscriber relationship without keeping either actor alive.
#[derive(Debug)]
pub struct SubscriptionReceiver {
    sender: Sender<Received>,
    messages: Receiver<Received>,
}

impl Filter {
    /// Select frames by APS cluster, ZCL command scope, and command direction.
    #[must_use]
    pub const fn new(cluster: Cluster, scope: Scope, direction: Direction) -> Self {
        Self {
            cluster,
            scope,
            direction,
        }
    }

    fn matches<T, U, K>(self, indication: &DataIndication<Frame<T>, U, K>) -> bool {
        let control = indication.asdu().header().control();

        indication.metadata().cluster_id() == self.cluster.as_u16()
            && control.typ() == Ok(self.scope)
            && control.direction() == self.direction
    }
}

impl Subscription {
    /// Create a bounded subscription channel for the given filter.
    pub fn channel(filter: Filter) -> (Self, SubscriptionReceiver) {
        let (sender, messages) = tokio::sync::mpsc::channel(MPSC_CHANNEL_SIZE);
        let subscription = Self {
            filter,
            messages: sender.downgrade(),
        };
        let receiver = SubscriptionReceiver { sender, messages };
        (subscription, receiver)
    }

    pub(super) fn is_open(&self) -> bool {
        self.messages
            .upgrade()
            .is_some_and(|messages| !messages.is_closed())
    }

    pub(super) fn same_channel(&self, other: &Sender<Received>) -> bool {
        self.messages
            .upgrade()
            .is_some_and(|messages| messages.same_channel(other))
    }

    pub(super) fn matches(&self, indication: &DataIndication<Frame<ZclCluster>, (), ()>) -> bool {
        self.filter.matches(indication)
    }

    pub(crate) fn try_send(&self, received: Received) -> Result<(), TrySendError<Received>> {
        let Some(messages) = self.messages.upgrade() else {
            return Err(TrySendError::Closed(received));
        };
        messages.try_send(received)
    }
}

impl SubscriptionReceiver {
    /// Clone a sending handle that identifies this subscription channel.
    pub(crate) fn sender(&self) -> Sender<Received> {
        self.sender.clone()
    }

    /// Receive the next frame delivered to this subscription.
    pub async fn recv(&mut self) -> Option<Received> {
        self.messages.recv().await
    }

    #[cfg(test)]
    pub fn try_recv(&mut self) -> Result<Received, TryRecvError> {
        self.messages.try_recv()
    }
}

#[cfg(test)]
mod tests {
    use zb_aps::apsde::{
        DataIndication, IndicationMetadata, IndicationStatus, IndividualEndpoint, NetworkAddress,
        ReceivedDestination, Security, Source,
    };
    use zb_core::endpoint::Application;
    use zb_core::{Cluster, Direction, Endpoint, Profile};
    use zb_zcl::{Frame, Header as ZclHeader, Scope};

    use super::{Filter, Subscription};

    const CLUSTER: Cluster = Cluster::OtaUpgrade;
    const OTHER_CLUSTER: Cluster = Cluster::OnOff;
    const LOCAL_ADDRESS: u16 = 0;
    const REMOTE_ADDRESS: u16 = 1;
    const LINK_QUALITY: u8 = u8::MAX;

    #[test]
    fn filter_matches_all_selected_header_fields() {
        let filter = Filter::new(CLUSTER, Scope::ClusterSpecific, Direction::ClientToServer);

        assert!(filter.matches(&indication(
            CLUSTER,
            Scope::ClusterSpecific,
            Direction::ClientToServer
        )));
        assert!(!filter.matches(&indication(
            OTHER_CLUSTER,
            Scope::ClusterSpecific,
            Direction::ClientToServer
        )));
        assert!(!filter.matches(&indication(
            CLUSTER,
            Scope::Global,
            Direction::ClientToServer
        )));
        assert!(!filter.matches(&indication(
            CLUSTER,
            Scope::ClusterSpecific,
            Direction::ServerToClient
        )));
    }

    #[test]
    fn subscription_does_not_keep_its_receiver_alive() {
        let (subscription, receiver) = Subscription::channel(Filter::new(
            CLUSTER,
            Scope::ClusterSpecific,
            Direction::ClientToServer,
        ));

        assert!(subscription.messages.upgrade().is_some());
        drop(receiver);
        assert!(subscription.messages.upgrade().is_none());
    }

    #[test]
    fn subscriptions_compare_by_channel() {
        let filter = Filter::new(CLUSTER, Scope::ClusterSpecific, Direction::ClientToServer);
        let (subscription, receiver) = Subscription::channel(filter);
        let (other_subscription, _other_receiver) = Subscription::channel(filter);
        let messages = receiver.sender();

        assert!(subscription.same_channel(&messages));
        assert!(!other_subscription.same_channel(&messages));
    }

    fn indication(
        cluster: Cluster,
        scope: Scope,
        direction: Direction,
    ) -> DataIndication<Frame<()>, (), ()> {
        let endpoint = Endpoint::Application(Application::MIN);
        let individual_endpoint =
            IndividualEndpoint::new(endpoint).expect("application endpoint is individual");
        let metadata = IndicationMetadata::new(
            ReceivedDestination::Network {
                address: NetworkAddress::new(LOCAL_ADDRESS).expect("coordinator address is valid"),
                endpoint: individual_endpoint,
            },
            Source::Network {
                address: NetworkAddress::new(REMOTE_ADDRESS).expect("device address is valid"),
                endpoint: individual_endpoint,
            },
            Profile::ZigbeeHomeAutomation.as_u16(),
            cluster.as_u16(),
            IndicationStatus::success(),
            Security::<()>::Unsecured,
            LINK_QUALITY,
            (),
        );
        let zcl_header = ZclHeader::new(scope, direction, false, None, 0, 0);
        DataIndication::new(metadata, Frame::new(zcl_header, ()))
    }
}
