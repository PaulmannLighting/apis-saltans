use std::fmt::Debug;

use apis_saltans_core::destination::Device;
use apis_saltans_core::{ClusterSpecific, Destination, ExpectResponse, Profiled};
use apis_saltans_zcl::{Cluster, Command};
use le_stream::ToLeStream;
use log::trace;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;

use super::Message;
use super::message::Payload;
use crate::Error;

/// Handle trait on the ZCL transceiver.
pub trait Handle {
    /// Send a ZCL command to a group of devices,
    /// where the command is a native ZCL command belonging to a static cluster.
    async fn transmit<T>(&self, destination: Destination, payload: T) -> Result<(), Error>
    where
        T: ClusterSpecific + Command + Debug + Profiled + ToLeStream;

    /// Communicate with a ZCL device's endpoint.
    fn communicate<T>(
        &self,
        destination: Device,
        payload: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<Cluster> + Into<Payload> + Send;
}

impl Handle for Sender<Message> {
    async fn transmit<T>(&self, destination: Destination, payload: T) -> Result<(), Error>
    where
        T: ClusterSpecific + Command + Debug + Profiled + ToLeStream,
    {
        let (response, result) = channel();
        trace!("Sending unicast message to {destination} with payload: {payload:?}");
        self.send(Message::Transmit {
            destination,
            payload: payload.into(),
            response,
        })
        .await?;
        Ok(result.await??)
    }

    fn communicate<T>(
        &self,
        destination: Device,
        payload: T,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<Cluster> + Into<Payload> + Send,
    {
        let (response, result) = channel();

        async move {
            self.send(Message::Communicate {
                destination,
                payload: payload.into(),
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
