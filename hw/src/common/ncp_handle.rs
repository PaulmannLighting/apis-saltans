#[cfg(feature = "driver")]
use std::num::NonZeroUsize;
#[cfg(feature = "coordinator")]
use std::time::Duration;

#[cfg(feature = "coordinator")]
use bytes::Bytes;
use tokio::sync::mpsc::{Receiver, Sender, WeakSender};
#[cfg(feature = "coordinator")]
use tokio::sync::oneshot::channel;
#[cfg(feature = "coordinator")]
use zb_aps::apsde::DataRequest;
#[cfg(feature = "coordinator")]
use zb_core::IeeeAddress;
#[cfg(feature = "coordinator")]
use zb_core::short_id::Device;
#[cfg(feature = "coordinator")]
use zb_zdp::SimpleDescriptor;

use super::message::Message;
#[cfg(feature = "coordinator")]
use super::message::RestoreRequest;
#[cfg(feature = "coordinator")]
use super::message::{ChannelMask, FoundNetwork, ScanDuration, ScannedChannel};
#[cfg(feature = "coordinator")]
use crate::{BackupCapabilities, Error, NetworkBackup, RestoreOptions, RestoreReport};

/// A handle on the NCP driver actor.
#[derive(Clone, Debug)]
pub struct NcpHandle {
    sender: Sender<Message>,
}

impl NcpHandle {
    #[cfg(feature = "driver")]
    pub(crate) fn channel(capacity: NonZeroUsize) -> (Self, Receiver<Message>) {
        let (sender, receiver) = tokio::sync::mpsc::channel(capacity.get());
        let handle = Self { sender };
        (handle, receiver)
    }

    #[cfg(feature = "coordinator")]
    async fn send(&self, message: Message) -> Result<(), Error> {
        Ok(self.sender.send(message).await?)
    }

    /// Create a weak handle that does not keep the driver actor channel open.
    #[must_use]
    pub fn downgrade(&self) -> WeakNcpHandle {
        WeakNcpHandle(self.sender.downgrade())
    }

    /// Return whether the driver actor channel has closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }

    /// Discover dump and restore operations registered at actor construction.
    /// Extension compatibility is validated against the target during restoration.
    ///
    /// # Errors
    /// Returns `Error::ActorUnavailable` if the actor has stopped.
    #[cfg(feature = "coordinator")]
    pub async fn backup_capabilities(&self) -> Result<BackupCapabilities, Error> {
        let (response, receiver) = channel();
        self.send(Message::GetBackupCapabilities { response })
            .await?;
        receiver.await?
    }

    /// Capture a complete core Zigbee network snapshot, including security secrets.
    /// Application-level state is excluded. Protect the returned backup at rest and in transit.
    ///
    /// # Errors
    /// Returns unsupported, actor-unavailable, or backend snapshot errors.
    #[cfg(feature = "coordinator")]
    pub async fn dump_network(&self) -> Result<NetworkBackup, Error> {
        let (response, receiver) = channel();
        self.send(Message::DumpNetwork { response }).await?;
        receiver.await?
    }

    /// Restore core network state on an offline target, leaving its network stopped.
    ///
    /// Dispatch advances outgoing counters before target validation and writes. The backend must
    /// persist and verify state before reporting success. Dropping this future after enqueueing
    /// does not cancel restoration. A lost response leaves the outcome unknown; verify before use.
    /// Stop the coordinator protocol actors before restoring and resume via backend startup.
    ///
    /// # Errors
    /// Returns unsupported, validation, target compatibility, counter, or hardware restore errors.
    /// Hardware failures identify whether persistent state may have changed.
    #[cfg(feature = "coordinator")]
    pub async fn restore_network(
        &self,
        backup: NetworkBackup,
        options: RestoreOptions,
    ) -> Result<RestoreReport, Error> {
        let (response, receiver) = channel();
        let request = Box::new(RestoreRequest { backup, options });
        self.send(Message::RestoreNetwork { request, response })
            .await?;
        receiver.await?
    }

    /// Return the local application endpoints provided by the NCP.
    ///
    /// The returned descriptors contain the endpoint ID, profile, device ID, application version,
    /// and input and output cluster lists advertised by each local endpoint. Coordinator-level ZDP
    /// and binding operations use these descriptors as the authoritative local endpoint set.
    ///
    /// # Errors
    ///
    /// Returns an error if the driver actor is unavailable or the NCP cannot provide its local
    /// endpoint descriptors.
    #[cfg(feature = "coordinator")]
    pub async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, Error> {
        let (response, receiver) = channel();
        self.send(Message::GetEndpoints { response }).await?;
        receiver.await?
    }

    /// Get the PAN ID of the coordinator's current network.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[cfg(feature = "coordinator")]
    pub async fn get_pan_id(&self) -> Result<u16, Error> {
        let (response, receiver) = channel();
        self.send(Message::GetPanId { response }).await?;
        receiver.await?
    }

    /// Get the IEEE address of the network manager.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[cfg(feature = "coordinator")]
    pub async fn get_ieee_address(&self) -> Result<IeeeAddress, Error> {
        let (response, receiver) = channel();
        self.send(Message::GetIeeeAddress { response }).await?;
        receiver.await?
    }

    /// Scan for available networks.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[cfg(feature = "coordinator")]
    pub async fn scan_networks(
        &self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> Result<Vec<FoundNetwork>, Error> {
        let (response, receiver) = channel();
        self.send(Message::ScanNetworks {
            channel_mask,
            duration,
            response,
        })
        .await?;
        receiver.await?
    }

    /// Scan channels for activity.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[cfg(feature = "coordinator")]
    pub async fn scan_channels(
        &self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> Result<Vec<ScannedChannel>, Error> {
        let (response, receiver) = channel();
        self.send(Message::ScanChannels {
            channel_mask,
            duration,
            response,
        })
        .await?;
        receiver.await?
    }

    /// Allow devices to join the network for the specified duration.
    ///
    /// # Returns
    ///
    /// Returns the actual duration for which joining is allowed. This may be less than the
    /// requested duration if the request exceeds the backend's maximum.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[cfg(feature = "coordinator")]
    pub async fn allow_joins(&self, duration: Duration) -> Result<Duration, Error> {
        let (response, receiver) = channel();
        self.send(Message::AllowJoins { duration, response })
            .await?;
        receiver.await?
    }

    /// Send a route request with the specified radius.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[cfg(feature = "coordinator")]
    pub async fn route_request(&self, radius: u8) -> Result<(), Error> {
        let (response, receiver) = channel();
        self.send(Message::RouteRequest { radius, response })
            .await?;
        receiver.await?
    }

    /// Get the IEEE address of the device with the specified short ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[cfg(feature = "coordinator")]
    pub async fn short_id_to_ieee_address(&self, short_id: Device) -> Result<IeeeAddress, Error> {
        let (response, receiver) = channel();
        self.send(Message::TranslateIeeeAddress { short_id, response })
            .await?;
        receiver.await?
    }

    /// Get the short ID of the device with the specified IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    #[cfg(feature = "coordinator")]
    pub async fn ieee_address_to_short_id(
        &self,
        ieee_address: IeeeAddress,
    ) -> Result<Device, Error> {
        let (response, receiver) = channel();
        self.send(Message::TranslateShortId {
            ieee_address,
            response,
        })
        .await?;
        receiver.await?
    }

    /// Submit an APS data-service request with its assigned frame counter.
    ///
    /// Success means the hardware backend accepted the request. APS completion is reported
    /// independently through [`crate::ApsdeEvent::DataConfirm`].
    ///
    /// # Errors
    ///
    /// Returns an error if the command cannot be handed to the driver actor or the backend rejects
    /// the request.
    #[cfg(feature = "coordinator")]
    pub async fn transmit(&self, request: DataRequest<Bytes>, counter: u8) -> Result<(), Error> {
        let (response, receiver) = channel();
        self.send(Message::Transmit {
            request,
            counter,
            response,
        })
        .await?;
        receiver.await??;
        Ok(())
    }
}

/// A weak handle on the NCP that does not keep the driver actor channel open.
#[derive(Clone, Debug)]
pub struct WeakNcpHandle(WeakSender<Message>);

impl WeakNcpHandle {
    /// Attempt to upgrade this weak handle.
    #[must_use]
    pub fn upgrade(&self) -> Option<NcpHandle> {
        Some(NcpHandle {
            sender: self.0.upgrade()?,
        })
    }
}
