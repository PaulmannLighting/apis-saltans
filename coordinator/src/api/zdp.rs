use le_stream::ToLeStream;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_core::short_id::Device;
use zb_core::{ClusterSpecific, ExpectResponse};
use zb_zdp::Command;

use crate::zdp::{Message, Payload};
use crate::{CommunicationResponse, Coordinator, Error};

/// A deferred typed ZDP response.
///
/// Awaiting this future waits for the correlated ZDP command and converts it to `T`.
pub type ZdpResponse<T> = CommunicationResponse<Command, T>;

/// Trait for sending ZDP requests.
///
/// `Coordinator` implements this trait directly. The `Node`, `Endpoints`, and `Binding` traits are
/// convenience wrappers over this raw ZDP transport.
pub trait Zdp {
    /// Send a ZDP request to a device and wait for its typed response.
    ///
    /// The returned outer future queues the request, awaits its acknowledged APS transmission, and
    /// yields a [`ZdpResponse`]. Await that response separately to receive and convert the
    /// correlated ZDP response command.
    ///
    /// # Errors
    ///
    /// The outer future returns an [`Error`] if the request cannot be queued. Awaiting the returned
    /// [`ZdpResponse`] returns an [`Error`] if transmission or reception fails, or if the raw
    /// command cannot be converted into `T::Response`.
    fn communicate<T>(
        &self,
        device: Device,
        request: T,
    ) -> impl Future<Output = Result<ZdpResponse<T::Response>, Error>> + Send
    where
        T: ClusterSpecific + ExpectResponse<Command> + ToLeStream;
}

impl Zdp for Sender<Message> {
    fn communicate<T>(
        &self,
        device: Device,
        command: T,
    ) -> impl Future<Output = Result<ZdpResponse<T::Response>, Error>> + Send
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
            Ok(result.await??.into())
        }
    }
}

impl Zdp for Coordinator {
    fn communicate<T>(
        &self,
        device: Device,
        command: T,
    ) -> impl Future<Output = Result<ZdpResponse<T::Response>, Error>> + Send
    where
        T: ClusterSpecific + ExpectResponse<Command> + ToLeStream,
    {
        self.zdp.communicate(device, command)
    }
}
