use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zigbee::{Address, Cluster, Endpoint};
use zigbee_hw::Metadata;

use super::{Message, Payload};
use crate::Error;
use crate::expect::ZclCommand;

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
    fn communicate<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Payload,
    ) -> impl Future<Output = Result<T, Error>> + Send
    where
        zcl::Cluster: ZclCommand<T>;

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
        Ok(result.await??)
    }

    async fn communicate<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Payload,
    ) -> Result<T, Error>
    where
        zcl::Cluster: ZclCommand<T>,
    {
        let (response, result) = channel();
        self.send(Message::Communicate {
            address,
            endpoint,
            payload: payload.into(),
            response,
        })
        .await?;
        result
            .await??
            .await?
            .expect()
            .ok_or(Error::InvalidResponseType)
    }
}
