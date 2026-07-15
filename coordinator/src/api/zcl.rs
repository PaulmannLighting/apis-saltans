use le_stream::ToLeStream;
use log::trace;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_core::destination::Device;
use zb_core::{ClusterSpecific, Destination, ExpectResponse, Profiled};
use zb_zcl::{Cluster, Command, Directed};

use crate::zcl::{Message, Payload};
use crate::{Coordinator, Error};

/// Trait for sending ZCL commands.
///
/// `Coordinator` implements this trait directly. The higher-level cluster traits (`OnOff`,
/// `ColorControl`, `Level`, and `Attributes`) are built on top of it.
pub trait Zcl {
    /// Send a ZCL command without waiting for an application-level response.
    ///
    /// Use this for cluster commands that are transmitted as commands or group/broadcast messages.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued for the ZCL transceiver or the
    /// hardware transmission fails.
    fn transmit<T>(
        &self,
        destination: Destination,
        payload: T,
    ) -> impl Future<Output = Result<(), Error>> + Send
    where
        T: ClusterSpecific + Command + Directed + Profiled + ToLeStream;

    /// Send a ZCL command to a device endpoint and wait for its typed response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued, the hardware transmission fails, the
    /// response times out, or the response cannot be converted into `T::Response`.
    fn communicate<T>(
        &self,
        destination: Device,
        payload: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<Cluster> + Into<Payload>;
}

impl Zcl for Sender<Message> {
    fn transmit<T>(
        &self,
        destination: Destination,
        payload: T,
    ) -> impl Future<Output = Result<(), Error>> + Send
    where
        T: ClusterSpecific + Command + Directed + Profiled + ToLeStream,
    {
        let payload = payload.into();
        let (response, result) = channel();
        trace!("Sending unicast message to {destination} with payload: {payload:?}");
        async move {
            self.send(Message::Transmit {
                destination,
                payload,
                response,
            })
            .await?;
            Ok(result.await??)
        }
    }

    fn communicate<T>(
        &self,
        destination: Device,
        payload: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<Cluster> + Into<Payload>,
    {
        let payload = payload.into();
        let (response, result) = channel();

        async move {
            self.send(Message::Communicate {
                destination,
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

impl Zcl for Coordinator {
    fn transmit<T>(
        &self,
        destination: Destination,
        payload: T,
    ) -> impl Future<Output = Result<(), Error>> + Send
    where
        T: ClusterSpecific + Command + Directed + Profiled + ToLeStream,
    {
        self.zcl.transmit(destination, payload)
    }

    fn communicate<T>(
        &self,
        destination: Device,
        payload: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<Cluster> + Into<Payload>,
    {
        self.zcl.communicate(destination, payload)
    }
}
