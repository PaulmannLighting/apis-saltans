use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zigbee::{Cluster, Endpoint};
use zigbee_hw::{Error, Metadata};

use crate::transmitter::{Message, Payload};

/// Handle trait on the zigbee transmitter.
pub trait Handle {
    /// Send a unicast.
    // TODO: Maybe mark this `unsafe` and document invariants?
    fn unicast(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        metadata: Metadata,
        payload: Payload,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Send a unicast of a native ZCL command belonging to a static cluster.
    async fn unicast_zcl_native<T>(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        command: T,
    ) -> Result<(), Error>
    where
        T: Cluster + Into<zcl::Cluster>,
    {
        self.unicast(
            short_id,
            endpoint,
            Metadata::for_cluster::<T>(None, None),
            Payload::Zcl {
                manufacturer_code: None,
                payload: command.into(),
            },
        )
        .await
    }
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
