use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zigbee::{Address, Cluster, Endpoint, RespondsWith};
use zigbee_hw::Metadata;

use super::{Message, Payload};
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
        payload: Payload,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Communicate a unicast with an expected response.
    fn communicate<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        metadata: Metadata,
        manufacturer_code: Option<u16>,
        payload: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: Into<zcl::Cluster> + RespondsWith<Response: TryFrom<zcl::Cluster>>;

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

    fn communicate<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        metadata: Metadata,
        manufacturer_code: Option<u16>,
        payload: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: Into<zcl::Cluster> + RespondsWith<Response: TryFrom<zcl::Cluster>>,
    {
        let (response, result) = channel();
        let payload = Payload::new(metadata, manufacturer_code, payload.into()).into();

        async move {
            self.send(Message::Communicate {
                address,
                endpoint,
                payload,
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
