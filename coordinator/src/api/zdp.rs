use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_core::short_id::Device;
use zb_core::{ClusterSpecific, ExpectResponse};
use zb_zdp::Command;

use crate::zdp::{Message, Payload};
use crate::{Coordinator, Error};

/// Trait for sending ZDP requests.
///
/// `Coordinator` implements this trait directly. The `Node`, `Endpoints`, and `Binding` traits are
/// convenience wrappers over this raw ZDP transport.
pub trait Zdp {
    /// Send a ZDP request to a device and wait for its typed response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request cannot be queued, the hardware transmission fails, the
    /// response times out, or the response cannot be converted into `T::Response`.
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
