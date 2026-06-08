use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zdp::Command;
use zigbee::{Address, Endpoint};
use zigbee_hw::{Error, Metadata};

use super::Message;

/// Handle trait on the ZDP transceiver.
pub trait Handle {
    /// Send a unicast.
    // TODO: Maybe mark this `unsafe` and document invariants?
    fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        metadata: Metadata,
        command: Command,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Handle for Sender<Message> {
    async fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        metadata: Metadata,
        command: Command,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        self.send(Message::Unicast {
            address,
            endpoint,
            metadata,
            command: command.into(),
            response,
        })
        .await?;
        result.await?
    }
}
