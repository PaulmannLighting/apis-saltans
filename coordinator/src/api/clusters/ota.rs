use tokio::sync::mpsc::Sender;
use zb_aps::apsde::IndividualEndpoint;
use zb_core::FullAddress;

pub use crate::ota::CancellableOtaUpdate;
use crate::ota::{Image, Message, Update, UpdateTimeouts};
use crate::{Coordinator, Error};

/// API for scheduling OTA updates through the coordinator-owned server.
pub trait Ota {
    /// Offer `image` to one fully identified device endpoint from the selected local APS source
    /// endpoint and initiate the OTA discovery flow.
    ///
    /// The OTA exchange uses the Zigbee Home Automation application profile.
    /// `target` pins the client's IEEE identity to its current NWK short address. The coordinator
    /// resolves the source of every subsequent OTA request through the NCP and rejects the request
    /// if the resolved IEEE address differs.
    ///
    /// A later call for the same endpoint replaces the previously offered image. The returned
    /// future remains pending while the OTA exchange runs and resolves after the client reports
    /// success or the server observes a terminal update failure. Dropping the future cancels the
    /// offer and releases its transfer resources. [`CancellableOtaUpdate::cancel`] provides the
    /// equivalent explicit operation.
    ///
    /// This method uses [`UpdateTimeouts::default`]. Use [`Self::update_with_timeouts`] to select
    /// deadlines for an individual offer.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SendError`] if the update cannot be queued, [`Error::ReceiveError`] if the
    /// server stops before reporting an outcome, or [`Error::Ota`] when subscription registration
    /// or the update fails, or when the configured concurrent update-task limit has been reached.
    fn update(
        &self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
    ) -> impl Future<Output = Result<(), Error>> + CancellableOtaUpdate + Send {
        self.update_with_timeouts(
            target,
            target_endpoint,
            source_endpoint,
            image,
            UpdateTimeouts::default(),
        )
    }

    /// Offer `image` with explicit discovery, block-inactivity, and total-transfer deadlines.
    ///
    /// Dropping or explicitly cancelling the returned future cancels the offer.
    fn update_with_timeouts(
        &self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
        timeouts: UpdateTimeouts,
    ) -> impl Future<Output = Result<(), Error>> + CancellableOtaUpdate + Send;
}

impl Ota for Sender<Message> {
    fn update_with_timeouts(
        &self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
        timeouts: UpdateTimeouts,
    ) -> impl Future<Output = Result<(), Error>> + CancellableOtaUpdate + Send {
        Update::new(
            self.clone(),
            target,
            target_endpoint,
            source_endpoint,
            image,
            timeouts,
        )
    }
}

impl Ota for Coordinator {
    fn update_with_timeouts(
        &self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
        timeouts: UpdateTimeouts,
    ) -> impl Future<Output = Result<(), Error>> + CancellableOtaUpdate + Send {
        self.ota
            .update_with_timeouts(target, target_endpoint, source_endpoint, image, timeouts)
    }
}
