use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zigbee::Endpoint;
use zigbee_hw::{Error, Metadata};

use crate::transmitter::{Message, Payload};

/// Handle trait on the zigbee transmitter.
pub trait Handle {
    /// Send a unicast.
    fn unicast(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        metadata: Metadata,
        payload: Payload,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Handle for Sender<Message> {
    async fn unicast(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        metadata: Metadata,
        payload: Payload,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        self.send(Message::Unicast {
            short_id,
            endpoint,
            metadata,
            payload: Box::new(payload),
            response,
        })
        .await?;
        result.await?
    }
}
