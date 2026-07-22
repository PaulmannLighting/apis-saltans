use std::future::Future;

use tokio::sync::mpsc::Sender;

use crate::ota::{Image, Message, Target};
use crate::{Coordinator, Error};

/// API for scheduling OTA updates through the coordinator-owned server.
pub trait Ota {
    /// Offer `image` to one device endpoint and initiate the OTA discovery flow.
    ///
    /// A later call for the same endpoint replaces the previously offered image. The returned
    /// future only confirms that the request reached the server actor; transmission and the
    /// remaining OTA exchange proceed asynchronously.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SendError`] if the OTA server actor is no longer running.
    fn update(
        &self,
        target: Target,
        image: Image,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Ota for Sender<Message> {
    async fn update(&self, target: Target, image: Image) -> Result<(), Error> {
        self.send(Message::Update { target, image }).await?;
        Ok(())
    }
}

impl Ota for Coordinator {
    async fn update(&self, target: Target, image: Image) -> Result<(), Error> {
        self.ota.update(target, image).await
    }
}
