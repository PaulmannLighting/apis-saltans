use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zb_aps::apsde::IndividualEndpoint;
use zb_core::FullAddress;

use crate::ota::{Image, Message, UpdateResult};
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
    /// success or the server observes a terminal update failure.
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
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Ota for Sender<Message> {
    async fn update(
        &self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
    ) -> Result<(), Error> {
        let (completion, result) = oneshot::channel::<UpdateResult>();
        self.send(Message::Update {
            target,
            target_endpoint,
            source_endpoint,
            image,
            completion,
        })
        .await?;
        result.await??;
        Ok(())
    }
}

impl Ota for Coordinator {
    async fn update(
        &self,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
    ) -> Result<(), Error> {
        self.ota
            .update(target, target_endpoint, source_endpoint, image)
            .await
    }
}
