use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zigbee::{Address, Cluster, Endpoint, ExpectResponse};
use zigbee_hw::Metadata;

use super::{Frame, Message};
use crate::Error;
use crate::timeout::Timeout;

/// Handle trait on the ZCL transceiver.
pub trait Handle {
    /// Send a unicast.
    // TODO: Maybe mark this `unsafe` and document invariants?
    fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Frame<zcl::Cluster>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Communicate a unicast with an expected response.
    fn communicate<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Frame<T>,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<zcl::Cluster>;

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
            Frame::new_native(Metadata::for_cluster::<T>(None, None), command.into()),
        )
        .await
    }
}

impl Handle for Sender<Message> {
    async fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Frame<zcl::Cluster>,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        self.send(Message::Unicast {
            address,
            endpoint,
            frame: payload.into(),
            response,
        })
        .await?;
        Ok(result.await??)
    }

    fn communicate<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Frame<T>,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<zcl::Cluster>,
    {
        let (response, result) = channel();
        let payload = payload.into_cluster().into();

        async move {
            self.send(Message::Communicate {
                address,
                endpoint,
                frame: payload,
                response,
            })
            .await?;
            result
                .await??
                .zcl_response_timeout()
                .await??
                .try_into()
                .map_err(|_| Error::InvalidResponseType)
        }
    }
}
