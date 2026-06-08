use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zigbee::{Address, Cluster, Endpoint};
use zigbee_hw::{Error, Metadata};

use super::Message;

/// Handle trait on the ZCL transceiver.
pub trait Handle {
    /// Send a unicast.
    // TODO: Maybe mark this `unsafe` and document invariants?
    fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        metadata: Metadata,
        manufacturer_code: Option<u16>,
        payload: zcl::Cluster,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Send a unicast of a native ZCL command belonging to a static cluster.
    async fn unicast_zcl_native<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        command: T,
    ) -> Result<(), Error>
    where
        T: Cluster + Into<zcl::Cluster>,
    {
        self.unicast(
            address,
            endpoint,
            Metadata::for_cluster::<T>(None, None),
            None,
            command.into(),
        )
        .await
    }
}

impl Handle for Sender<Message> {
    async fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        metadata: Metadata,
        manufacturer_code: Option<u16>,
        payload: zcl::Cluster,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        self.send(Message::Unicast {
            address,
            endpoint,
            metadata,
            manufacturer_code,
            payload: payload.into(),
            response,
        })
        .await?;
        result.await?
    }
}
