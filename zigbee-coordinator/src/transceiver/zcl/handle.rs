use std::borrow::Borrow;

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zigbee::{Cluster, Endpoint, ExpectResponse};
use zigbee_hw::Metadata;

use super::{Message, Payload};
use crate::Error;
use crate::timeout::Timeout;

/// Handle trait on the ZCL transceiver.
pub trait Handle {
    /// Send a ZCL command to one specific device and endpoint.
    fn unicast(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        payload: Payload<zcl::Cluster>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Communicate with a ZCL device's endpoint.
    fn communicate<T>(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        payload: Payload<T>,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<zcl::Cluster>;

    /// Send a ZCL command to one specific device and endpoint,
    /// where the command is a native ZCL command belonging to a static cluster.
    async fn unicast_static_cluster<T>(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        command: T,
    ) -> Result<(), Error>
    where
        T: Cluster + Into<zcl::Cluster>,
    {
        #[expect(unsafe_code)]
        // SAFETY: We construct matching metadata from the payload type.
        let payload =
            unsafe { Payload::new_native(Metadata::for_cluster::<T>(None, None), command.into()) };
        self.unicast(short_id, endpoint, payload).await
    }
}

impl<T> Handle for T
where
    T: Borrow<Sender<Message>> + Sync,
{
    async fn unicast(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        payload: Payload<zcl::Cluster>,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        self.borrow()
            .send(Message::Unicast {
                short_id,
                endpoint,
                payload: payload.into(),
                response,
            })
            .await?;
        Ok(result.await??)
    }

    fn communicate<U>(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        payload: Payload<U>,
    ) -> impl Future<Output = Result<U::Response, Error>> + Send
    where
        U: ExpectResponse<zcl::Cluster>,
    {
        let (response, result) = channel();
        let payload = payload.into_cluster().into();

        async move {
            self.borrow()
                .send(Message::Communicate {
                    short_id,
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
                .map_err(|error| Error::InvalidResponseType(format!("{error:?}")))
        }
    }
}
