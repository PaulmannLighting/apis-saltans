use std::borrow::Borrow;

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zdp::Command;
use zigbee::{Cluster, ExpectResponse};

use super::Message;
use crate::Error;
use crate::timeout::Timeout;

/// Handle trait on the ZDP transceiver.
pub trait Handle {
    /// Communicate a unicast with an expected response.
    fn communicate<T>(
        &self,
        short_id: u16,
        request: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
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
        command: U,
    ) -> impl Future<Output = Result<U::Response, Error>> + Send
    where
        U: Cluster + ExpectResponse<Command>,
    {
        let (response, result) = channel();
        let command = Box::new(command.into());

        async move {
            self.borrow()
                .send(Message::Communicate {
                    short_id,
                    command,
                    response,
                })
                .await?;
            result
                .await??
                .zdp_response_timeout()
                .await??
                .try_into()
                .map_err(|error| Error::InvalidResponseType(format!("{error:?}")))
        }
    }
}
