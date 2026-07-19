use le_stream::ToLeStream;
use log::trace;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;
use zb_core::destination::Device;
use zb_core::{ClusterSpecific, Destination, ExpectResponse, Profiled};
use zb_hw::HwResponse;
use zb_zcl::{Cluster, Command, Directed};

use crate::zcl::{Message, Payload};
use crate::{CommunicationResponse, Coordinator, Error};

/// A deferred typed ZCL response.
///
/// Awaiting this future first confirms hardware transmission, then waits for the correlated ZCL
/// frame and converts it to `T`.
pub type ZclResponse<T> = CommunicationResponse<Cluster, T>;

/// Trait for sending ZCL commands.
///
/// `Coordinator` implements this trait directly. The higher-level cluster traits (`OnOff`,
/// `ColorControl`, `Level`, and `Attributes`) are built on top of it.
pub trait Zcl {
    /// Send a ZCL command without waiting for an application-level response.
    ///
    /// Use this for cluster commands that are transmitted as commands or group/broadcast messages.
    /// The returned outer future queues the command and yields an [`HwResponse`]. Await that
    /// response separately to observe whether the hardware transmission completed.
    ///
    /// # Errors
    ///
    /// The outer future returns an [`Error`] if the command cannot be queued. Awaiting the returned
    /// [`HwResponse`] returns a [`zb_hw::Error`] if the deferred hardware transmission fails.
    fn transmit<T>(
        &self,
        destination: Destination,
        payload: T,
    ) -> impl Future<Output = Result<HwResponse, Error>> + Send
    where
        T: ClusterSpecific + Command + Directed + Profiled + ToLeStream;

    /// Send a ZCL command to a device endpoint and wait for its typed response.
    ///
    /// The returned outer future queues the request and yields a [`ZclResponse`]. Await that
    /// response separately; it confirms hardware transmission before waiting for and converting
    /// the correlated ZCL response frame.
    ///
    /// # Errors
    ///
    /// The outer future returns an [`Error`] if the command cannot be queued. Awaiting the returned
    /// [`ZclResponse`] returns an [`Error`] if transmission or reception fails, or if the raw frame
    /// cannot be converted into `T::Response`.
    fn communicate<T>(
        &self,
        destination: Device,
        payload: T,
    ) -> impl Future<Output = Result<ZclResponse<T::Response>, Error>> + Send
    where
        T: ExpectResponse<Cluster> + Into<Payload>;
}

impl Zcl for Sender<Message> {
    fn transmit<T>(
        &self,
        destination: Destination,
        payload: T,
    ) -> impl Future<Output = Result<HwResponse, Error>> + Send
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
        device: Device,
        payload: T,
    ) -> impl Future<Output = Result<ZclResponse<T::Response>, Error>> + Send
    where
        T: ExpectResponse<Cluster> + Into<Payload>,
    {
        let payload = payload.into();
        let (response, result) = channel();

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

impl Zcl for Coordinator {
    fn transmit<T>(
        &self,
        destination: Destination,
        payload: T,
    ) -> impl Future<Output = Result<HwResponse, Error>> + Send
    where
        T: ClusterSpecific + Command + Directed + Profiled + ToLeStream,
    {
        self.zcl.transmit(destination, payload)
    }

    fn communicate<T>(
        &self,
        destination: Device,
        payload: T,
    ) -> impl Future<Output = Result<ZclResponse<T::Response>, Error>> + Send
    where
        T: ExpectResponse<Cluster> + Into<Payload>,
    {
        self.zcl.communicate(destination, payload)
    }
}
