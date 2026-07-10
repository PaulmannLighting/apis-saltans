use std::borrow::Borrow;

use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_core::short_id::Device;
use zb_core::{ClusterSpecific, ExpectResponse};
use zb_zdp::Command;

use super::{Message, Payload};
use crate::Error;

/// Handle trait on the ZDP transceiver.
pub trait Handle {
    /// Communicate a unicast with an expected response.
    fn communicate<T>(
        &self,
        device: Device,
        request: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ClusterSpecific + ExpectResponse<Command> + ToLeStream;
}

impl<T> Handle for T
where
    T: Borrow<Sender<Message>> + Sync,
{
    fn communicate<U>(
        &self,
        device: Device,
        command: U,
    ) -> impl Future<Output = Result<U::Response, Error>> + Send
    where
        U: ClusterSpecific + ExpectResponse<Command> + ToLeStream,
    {
        let (response, result) = channel();
        let payload = Payload::from(command);

        async move {
            self.borrow()
                .send(Message::Communicate {
                    device,
                    payload,
                    response,
                })
                .await?;
            result
                .await??
                .await?
                .try_into()
                .map_err(|error| Error::InvalidResponseType(format!("{error:?}")))
        }
    }
}
