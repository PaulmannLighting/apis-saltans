use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use crate::ota::{Image, Message, Target, UpdateResult};
use crate::{Coordinator, Error};

/// API for scheduling OTA updates through the coordinator-owned server.
pub trait Ota {
    /// Offer `image` to one device endpoint and initiate the OTA discovery flow.
    ///
    /// A later call for the same endpoint replaces the previously offered image. The returned
    /// future remains pending while the OTA exchange runs and resolves after the client reports
    /// success or the server observes a terminal update failure.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SendError`] if the update cannot be queued, [`Error::ReceiveError`] if the
    /// server stops before reporting an outcome, or [`Error::Ota`] when the update fails.
    fn update(
        &self,
        target: Target,
        image: Image,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Ota for Sender<Message> {
    async fn update(&self, target: Target, image: Image) -> Result<(), Error> {
        let (completion, result) = oneshot::channel::<UpdateResult>();
        self.send(Message::Update {
            target,
            image,
            completion,
        })
        .await?;
        result.await??;
        Ok(())
    }
}

impl Ota for Coordinator {
    async fn update(&self, target: Target, image: Image) -> Result<(), Error> {
        self.ota.update(target, image).await
    }
}
