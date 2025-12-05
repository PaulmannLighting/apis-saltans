use std::collections::BTreeMap;
use std::time::Duration;

use macaddr::MacAddr8;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zigbee::Endpoint;

use crate::actor::Message;
use crate::device_proxy::DeviceProxy;
use crate::zcl_proxy::ZclProxy;
use crate::{Error, Frame};

/// Proxy trait for sending NWK layer messages.
pub trait Proxy {
    /// Get the next transaction sequence number.
    fn get_transaction_seq(&self) -> impl Future<Output = u8>;

    /// Get the PAN ID of the network manager.
    fn get_pan_id(&self) -> impl Future<Output = Result<u16, Error>>;

    /// Allow devices to join the network for the specified duration.
    fn allow_joins(&self, duration: Duration) -> impl Future<Output = Result<(), Error>>;

    /// Get the list of neighbor devices.
    fn get_neighbors(&self) -> impl Future<Output = Result<BTreeMap<MacAddr8, u16>, Error>>;

    /// Send a unicast ZCL command.
    fn unicast(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        cluster_id: u16,
        group_id: u16,
        frame: Frame,
    ) -> impl Future<Output = Result<(), Error>>;

    /// Get a device proxy for the specified PAN ID.
    fn device(&self, pan_id: u16) -> DeviceProxy<'_, Self>
    where
        Self: Sized,
    {
        DeviceProxy::new(self, pan_id)
    }

    /// Get a ZCL proxy.
    fn zcl(&self) -> ZclProxy<'_, Self>
    where
        Self: Sized,
    {
        ZclProxy::new(self)
    }
}

impl Proxy for Sender<Message> {
    async fn get_transaction_seq(&self) -> u8 {
        let (response, rx) = oneshot::channel();
        self.send(Message::GetTransactionSeq { response })
            .await
            .map_err(|_| Error::ActorSend)
            .unwrap();
        rx.await.map_err(|_| Error::ActorReceive).unwrap()
    }

    async fn get_pan_id(&self) -> Result<u16, Error> {
        let (response, rx) = oneshot::channel();
        self.send(Message::GetPanId { response })
            .await
            .map_err(|_| Error::ActorSend)?;
        rx.await.map_err(|_| Error::ActorReceive)?
    }

    async fn allow_joins(&self, duration: Duration) -> Result<(), Error> {
        let (response, rx) = oneshot::channel();
        self.send(Message::AllowJoins { duration, response })
            .await
            .map_err(|_| Error::ActorSend)?;
        rx.await.map_err(|_| Error::ActorReceive)?
    }

    async fn get_neighbors(&self) -> Result<BTreeMap<MacAddr8, u16>, Error> {
        let (response, rx) = oneshot::channel();
        self.send(Message::GetNeighbors { response })
            .await
            .map_err(|_| Error::ActorSend)?;
        rx.await.map_err(|_| Error::ActorReceive)?
    }

    async fn unicast(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        cluster_id: u16,
        group_id: u16,
        frame: Frame,
    ) -> Result<(), Error> {
        let (response, rx) = oneshot::channel();
        self.send(Message::Unicast {
            pan_id,
            endpoint,
            cluster_id,
            group_id,
            frame,
            response,
        })
        .await
        .map_err(|_| Error::ActorSend)?;
        rx.await.map_err(|_| Error::ActorReceive)?
    }
}
