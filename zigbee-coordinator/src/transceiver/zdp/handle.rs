use std::borrow::Borrow;

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zdp::Command;
use zigbee::{Cluster, ExpectResponse};
use zigbee_hw::Metadata;

use super::{Message, Payload};
use crate::timeout::Timeout;

/// Handle trait on the ZDP transceiver.
pub trait Handle {
    /// Communicate a unicast with an expected response.
    fn communicate<T>(
        &self,
        short_id: u16,
        request: T,
    ) -> impl Future<Output = Result<T::Response, crate::Error>> + Send
    where
        T: Cluster + ExpectResponse<Command>;
}

impl<T> Handle for T
where
    T: Borrow<Sender<Message>> + Sync,
{
    fn communicate<U>(
        &self,
        short_id: u16,
        request: U,
    ) -> impl Future<Output = Result<U::Response, crate::Error>> + Send
    where
        U: Cluster + ExpectResponse<Command>,
    {
        let (response, result) = channel();
        let payload = Payload::new(Metadata::new(U::ID, None, None), request)
            .into_command()
            .into();

        async move {
            self.borrow()
                .send(Message::Communicate {
                    short_id,
                    payload,
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
