use std::collections::{BTreeMap, BTreeSet};

use aps::Data;
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::sync::oneshot;
use zcl::{Cluster, Frame};
use zigbee::Address;

use super::message::Message;
use crate::{Device, Error, Event};

/// Handle trait on the network manager.
pub trait Handle {
    /// Subscribe to network events.
    fn subscribe(
        &self,
        devices: BTreeSet<MacAddr8>,
        channel_size: usize,
    ) -> impl Future<Output = Result<Receiver<Event>, Error>> + Send;

    /// Send a ZCL command to one specific device and endpoint.
    fn command(
        &self,
        src_address: u16,
        payload: Box<Data<Frame<Cluster>>>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Return the IEEE address for the given short ID.
    fn ieee_address_to_short_id(&self, ieee_address: MacAddr8) -> Result<Option<u16>, Error>;

    /// Return the short ID for the given IEEE address.
    fn short_id_to_ieee_address(&self, short_id: u16) -> Result<Option<MacAddr8>, Error>;

    /// List known devices of the network manager.
    fn get_devices(&self) -> Result<BTreeMap<MacAddr8, Device>, Error>;

    /// Add a new device to the network.
    fn new_device(&self, device: Device) -> Result<(), Error>;

    /// Remove a device from the network.
    fn remove_device(&self, address: Address) -> Result<(), Error>;
}

impl Handle for Sender<Message> {
    async fn subscribe(
        &self,
        devices: BTreeSet<MacAddr8>,
        channel_size: usize,
    ) -> Result<Receiver<Event>, Error> {
        let (tx, rx) = channel(channel_size);
        self.send(Message::SubscribeToIncomingCommands {
            devices,
            sender: tx,
        })
        .await?;
        Ok(rx)
    }

    async fn command(
        &self,
        src_address: u16,
        payload: Box<Data<Frame<Cluster>>>,
    ) -> Result<(), Error> {
        Ok(self
            .send(Message::Command {
                src_address,
                payload,
            })
            .await?)
    }

    async fn ieee_address_to_short_id(&self, ieee_address: MacAddr8) -> Result<Option<u16>, Error> {
        let (tx, rx) = oneshot::channel();
        self.send(Message::GetShortIdFromIeeeAddress {
            ieee_address,
            response: tx,
        })
        .await?;
        Ok(rx.await?)
    }

    async fn short_id_to_ieee_address(&self, short_id: u16) -> Result<Option<MacAddr8>, Error> {
        let (tx, rx) = oneshot::channel();
        self.send(Message::GetIeeeAddressFromShortId {
            short_id,
            response: tx,
        })
        .await?;
        Ok(rx.await?)
    }

    async fn get_devices(&self) -> Result<BTreeMap<MacAddr8, Device>, Error> {
        let (tx, rx) = oneshot::channel();
        self.send(Message::GetDevices { response: tx }).await?;
        Ok(rx.await?)
    }

    async fn new_device(&self, device: Device) -> Result<(), Error> {
        Ok(self.send(Message::NewDevice(device)).await?)
    }

    async fn remove_device(&self, address: Address) -> Result<(), Error> {
        Ok(self.send(Message::RemoveDevice(address)).await?)
    }
}
