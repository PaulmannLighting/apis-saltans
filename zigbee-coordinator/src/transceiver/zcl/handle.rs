use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zigbee::{Address, Cluster, Endpoint};
use zigbee_hw::{Error, Metadata};

use super::{Message, Payload};

/// Handle trait on the ZCL transceiver.
pub trait Handle {
    /// Send a unicast.
    // TODO: Maybe mark this `unsafe` and document invariants?
    fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Payload,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Communicate a unicast with an expected response.
    fn communicate(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Payload,
    ) -> impl Future<Output = Result<zcl::Cluster, Error>> + Send;

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
            Payload::new_native(Metadata::for_cluster::<T>(None, None), command.into()),
        )
        .await
    }
}

impl Handle for Sender<Message> {
    async fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Payload,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        self.send(Message::Unicast {
            address,
            endpoint,
            payload: payload.into(),
            response,
        })
        .await?;
        result.await?
    }

    async fn communicate(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Payload,
    ) -> Result<zcl::Cluster, Error> {
        let (response, result) = channel();
        self.send(Message::Communicate {
            address,
            endpoint,
            payload: payload.into(),
            response,
        })
        .await?;
        Ok(result.await??.await?)
    }
}
