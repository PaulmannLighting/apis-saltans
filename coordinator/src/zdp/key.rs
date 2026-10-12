use zb_aps::apsde::{DataIndication, DataRequest, ReceivedDestination, Source};
use zb_core::Endpoint;
use zb_core::short_id::Device;
use zb_zdp::{CLUSTER_ID_RESPONSE_MASK, Command, Frame};

/// ZDP response identity; both endpoints must be the data endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZdpKey {
    /// Network short address of the remote node.
    pub short_id: u16,
    /// Request cluster id, without the response mask.
    pub cluster_id: u16,
    /// Profile used by the exchange.
    pub profile_id: u16,
    /// Transaction sequence number.
    pub sequence: u8,
}

impl ZdpKey {
    /// Create the response correlation key for a sent ZDP command.
    ///
    /// ZDP commands are exchanged on the data endpoint and do not carry a ZCL
    /// manufacturer code, so the key is built from the request's cluster and
    /// profile identifiers plus the transaction sequence number.
    #[must_use]
    pub fn from_command<T>(device: Device, seq: u8, request: &DataRequest<T>) -> Self {
        Self {
            short_id: device.into(),
            cluster_id: request.cluster_id(),
            profile_id: request.profile_id(),
            sequence: seq,
        }
    }

    /// Create the response correlation key for a received ZDP indication.
    ///
    /// ZDP response cluster ids carry [`CLUSTER_ID_RESPONSE_MASK`]. The mask is
    /// toggled away before indexing so the response matches the key that was
    /// stored for the original request command.
    ///
    /// Returns `None` unless both the source and destination use the ZDP data endpoint and the
    /// source is identified by a 16-bit NWK address.
    #[must_use]
    pub fn from_indication<T, K>(
        indication: &DataIndication<Frame<Command>, T, K>,
    ) -> Option<Self> {
        let Source::Network {
            address: source,
            endpoint: source_endpoint,
        } = indication.metadata().source()
        else {
            return None;
        };
        if source_endpoint.get() != Endpoint::Data {
            return None;
        }

        let destination_endpoint = match indication.metadata().destination() {
            ReceivedDestination::Broadcast { endpoint, .. } => endpoint,
            ReceivedDestination::Network { endpoint, .. }
            | ReceivedDestination::Extended { endpoint, .. } => endpoint.get(),
            ReceivedDestination::Group(_) | ReceivedDestination::ExtendedWithoutEndpoint(_) => {
                return None;
            }
        };
        if destination_endpoint != Endpoint::Data {
            return None;
        }

        Some(Self {
            short_id: source.as_u16(),
            cluster_id: indication.metadata().cluster_id() ^ CLUSTER_ID_RESPONSE_MASK,
            profile_id: indication.metadata().profile_id(),
            sequence: indication.asdu().seq(),
        })
    }
}

#[cfg(test)]
mod tests {
    use zb_aps::apsde::{
        DataIndication, IndicationMetadata, IndicationStatus, IndividualEndpoint, NetworkAddress,
        ReceivedDestination, Security, Source,
    };
    use zb_core::endpoint::Application;
    use zb_core::{Endpoint, Profile};
    use zb_zdp::{Command, Frame, MgmtPermitJoiningRsp, NetworkManagement, Status};

    use super::ZdpKey;

    const LOCAL_ADDRESS: u16 = 0;
    const REMOTE_ADDRESS: u16 = 1;
    const LINK_QUALITY: u8 = u8::MAX;
    const SEQUENCE: u8 = 42;

    #[test]
    fn received_zdp_indication_requires_endpoint_zero_at_both_ends() {
        assert!(ZdpKey::from_indication(&indication(Endpoint::Data, Endpoint::Data)).is_some());
        assert!(
            ZdpKey::from_indication(&indication(application_endpoint(), Endpoint::Data)).is_none()
        );
        assert!(
            ZdpKey::from_indication(&indication(Endpoint::Data, application_endpoint())).is_none()
        );
    }

    fn indication(
        source_endpoint: Endpoint,
        destination_endpoint: Endpoint,
    ) -> DataIndication<Frame<Command>, (), ()> {
        let command: Command =
            NetworkManagement::from(MgmtPermitJoiningRsp::new(Status::Success)).into();
        let metadata = IndicationMetadata {
            destination: ReceivedDestination::Network {
                address: network_address(LOCAL_ADDRESS),
                endpoint: individual_endpoint(destination_endpoint),
            },
            source: Source::Network {
                address: network_address(REMOTE_ADDRESS),
                endpoint: individual_endpoint(source_endpoint),
            },
            profile_id: Profile::Network.as_u16(),
            cluster_id: command.cluster_id(),
            status: IndicationStatus::success(),
            security: Security::<()>::Unsecured,
            link_quality: LINK_QUALITY,
            rx_time: (),
        };

        DataIndication::new(metadata, Frame::new(SEQUENCE, command))
    }

    fn application_endpoint() -> Endpoint {
        Endpoint::Application(Application::MIN)
    }

    fn individual_endpoint(endpoint: Endpoint) -> IndividualEndpoint {
        IndividualEndpoint::new(endpoint).expect("test endpoint must be individual")
    }

    fn network_address(address: u16) -> NetworkAddress {
        NetworkAddress::new(address).expect("test address must be individual")
    }
}
