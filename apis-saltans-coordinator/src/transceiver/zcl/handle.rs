use apis_saltans_core::{Application, Cluster, ExpectResponse};
use log::trace;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::channel;

use super::{Message, Payload};
use crate::{Destination, Error, NetworkManager};

/// Handle trait on the ZCL transceiver.
pub trait Handle {
    /// Send a ZCL command to one specific device and endpoint.
    fn unicast(
        &self,
        short_id: u16,
        endpoint: Application,
        payload: Payload<apis_saltans_zcl::Cluster>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Send a ZCL command to a group of devices.
    fn multicast(
        &self,
        group_id: u16,
        hops: u8,
        radius: u8,
        payload: Payload<apis_saltans_zcl::Cluster>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Communicate with a ZCL device's endpoint.
    fn communicate<T>(
        &self,
        short_id: u16,
        endpoint: Application,
        payload: Payload<T>,
    ) -> impl Future<Output = Result<T::Response, Error>> + Send
    where
        T: ExpectResponse<apis_saltans_zcl::Cluster>;

    /// Send a ZCL command to one specific device and endpoint,
    /// where the command is a native ZCL command belonging to a static cluster.
    async fn unicast_static_cluster<T>(
        &self,
        short_id: u16,
        endpoint: Application,
        command: T,
    ) -> Result<(), Error>
    where
        T: Cluster + Into<apis_saltans_zcl::Cluster>,
    {
        let payload = Payload::for_cluster(command);
        self.unicast(short_id, endpoint, payload).await
    }

    /// Send a ZCL command to a group of devices,
    /// where the command is a native ZCL command belonging to a static cluster.
    async fn multicast_static_cluster<T>(
        &self,
        group_id: u16,
        hops: u8,
        radius: u8,
        command: T,
    ) -> Result<(), Error>
    where
        T: Cluster + Into<apis_saltans_zcl::Cluster>,
    {
        let payload = Payload::for_cluster(command);
        self.multicast(group_id, hops, radius, payload).await
    }

    async fn send_static_cluster<T>(
        &self,
        destination: Destination,
        command: T,
    ) -> Result<(), Error>
    where
        Self: NetworkManager,
        T: Cluster + Into<apis_saltans_zcl::Cluster>,
    {
        match destination {
            Destination::Endpoint {
                ieee_address,
                endpoint,
            } => {
                let short_id = self
                    .get_short_id_from_ieee_address(ieee_address)
                    .await?
                    .ok_or(Error::UnknownDevice(ieee_address))?;
                self.unicast_static_cluster(short_id, endpoint, command)
                    .await
            }
            Destination::Group(group_id) => {
                self.multicast_static_cluster(group_id, 0, 0, command).await
            }
        }
    }
}

impl Handle for Sender<Message> {
    async fn unicast(
        &self,
        short_id: u16,
        endpoint: Application,
        payload: Payload<apis_saltans_zcl::Cluster>,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        trace!("Sending unicast message to {short_id:#06X}:{endpoint} with payload: {payload:?}");
        self.send(Message::Unicast {
            short_id,
            endpoint,
            payload: payload.into(),
            response,
        })
        .await?;
        Ok(result.await??)
    }

    async fn multicast(
        &self,
        group_id: u16,
        hops: u8,
        radius: u8,
        payload: Payload<apis_saltans_zcl::Cluster>,
    ) -> Result<(), Error> {
        let (response, result) = channel();
        trace!(
            "Sending multicast message to {group_id:#06X} with {hops} hops and within radius {radius} with payload: {payload:?}"
        );
        self.send(Message::Multicast {
            group_id,
            hops,
            radius,
            payload: payload.into(),
            response,
        })
        .await?;
        Ok(result.await??)
    }

    fn communicate<U>(
        &self,
        short_id: u16,
        endpoint: Application,
        payload: Payload<U>,
    ) -> impl Future<Output = Result<U::Response, Error>> + Send
    where
        U: ExpectResponse<apis_saltans_zcl::Cluster>,
    {
        let (response, result) = channel();
        let payload = payload.into_cluster().into();

        async move {
            self.send(Message::Communicate {
                short_id,
                endpoint,
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
