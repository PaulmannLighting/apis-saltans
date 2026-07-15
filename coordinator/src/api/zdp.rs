use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_core::short_id::Device;
use zb_core::{ClusterSpecific, ExpectResponse};
use zb_zdp::Command;

use crate::zdp::{Message, Payload};
use crate::{Coordinator, Error};

/// Handle trait on the ZDP transceiver.
pub trait Zdp {
    /// Communicate a unicast with an expected response.
    fn communicate<T>(
        &self,
        device: Device,
        request: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ClusterSpecific + ExpectResponse<Command> + ToLeStream;
}

impl Zdp for Sender<Message> {
    fn communicate<T>(
        &self,
        device: Device,
        command: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ClusterSpecific + ExpectResponse<Command> + ToLeStream,
    {
        let (response, result) = channel();
        let payload = Payload::from(command);

        async move {
            self.send(Message::Communicate {
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

impl Zdp for Coordinator {
    fn communicate<T>(
        &self,
        device: Device,
        command: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ClusterSpecific + ExpectResponse<Command> + ToLeStream,
    {
        self.zdp.communicate(device, command)
    }
}
