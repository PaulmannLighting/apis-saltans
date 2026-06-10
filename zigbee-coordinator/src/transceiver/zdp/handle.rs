use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zdp::Command;
use zigbee::{Address, Endpoint, ExpectResponse};
use zigbee_hw::Error;

use super::Message;
use crate::timeout::Timeout;
use crate::transceiver::aps::Frame;

/// Handle trait on the ZDP transceiver.
pub trait Handle {
    /// Send a unicast.
    // TODO: Maybe mark this `unsafe` and document invariants?
    fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        frame: Frame<Command>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Communicate a unicast with an expected response.
    fn communicate<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        frame: Frame<T>,
    ) -> impl Future<Output = Result<T::Response, crate::Error>> + Send
    where
        T: ExpectResponse<Command>;
}

impl Handle for Sender<Message> {
    async fn unicast(
        &self,
        address: Address,
        endpoint: Endpoint,
        frame: Frame<Command>,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        self.send(Message::Unicast {
            address,
            endpoint,
            frame: frame.into(),
            response,
        })
        .await?;
        result.await?
    }

    fn communicate<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        payload: Frame<T>,
    ) -> impl Future<Output = Result<T::Response, crate::Error>> + Send
    where
        T: ExpectResponse<Command>,
    {
        let (response, result) = channel();
        let payload = payload.into_command().into();

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
                .zdp_response_timeout()
                .await??
                .try_into()
                .map_err(|_| crate::Error::InvalidResponseType)
        }
    }
}
