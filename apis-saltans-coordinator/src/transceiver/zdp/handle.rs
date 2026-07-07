use std::borrow::Borrow;

use apis_saltans_core::{Cluster, ExpectResponse};
use apis_saltans_zdp::Command;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;

use super::Message;
use crate::Error;

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
        let command = command.into();

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
                .await?
                .try_into()
                .map_err(|error| Error::InvalidResponseType(format!("{error:?}")))
        }
    }
}
