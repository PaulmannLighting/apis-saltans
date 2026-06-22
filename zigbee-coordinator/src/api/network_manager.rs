use std::collections::{BTreeMap, BTreeSet};

use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;
use zigbee::Address;

use crate::network_manager::Message;
use crate::{Coordinator, Device, Error, Event};

/// Handle to the network manager actor.
pub trait NetworkManager {
    /// Return the IEEE address for the given short ID.
    ///
    /// # Returns
    ///
    /// Returns `Some(address)` if the short ID is known, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the actor failed.
    fn get_ieee_address_from_short_id(
        &self,
        short_id: u16,
    ) -> impl Future<Output = Result<Option<MacAddr8>, Error>> + Send;

    /// Return the short ID for the given IEEE address.
    ///
    /// # Returns
    ///
    /// Returns `Some(short_id)` if the IEEE address is known, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the actor failed.
    fn get_short_id_from_ieee_address(
        &self,
        ieee_address: MacAddr8,
    ) -> impl Future<Output = Result<Option<u16>, Error>> + Send;

    /// Resolve the given short ID into an [`Address`].
    ///
    /// # Returns
    ///
    /// Returns `Some(address)` if the address is known, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the actor failed.
    fn short_id_to_address(
        &self,
        short_id: u16,
    ) -> impl Future<Output = Result<Option<Address>, Error>> + Send
    where
        Self: Sync,
    {
        async move {
            self.get_ieee_address_from_short_id(short_id)
                .await
                .map(|result| result.map(|ieee_address| Address::new(ieee_address, short_id)))
        }
    }

    /// Resolve the given IEEE address into an [`Address`].
    ///
    /// # Returns
    ///
    /// Returns `Some(address)` if the address is known, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the actor failed.
    fn ieee_address_to_address(
        &self,
        ieee_address: MacAddr8,
    ) -> impl Future<Output = Result<Option<Address>, Error>> + Send
    where
        Self: Sync,
    {
        async move {
            self.get_short_id_from_ieee_address(ieee_address)
                .await
                .map(|result| result.map(|short_id| Address::new(ieee_address, short_id)))
        }
    }

    /// List known devices of the network manager.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the actor failed.
    fn state(&self) -> impl Future<Output = Result<BTreeMap<MacAddr8, Device>, Error>>;

    /// Subscribes to a stream of incoming commands.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the actor failed.
    fn subscribe_to_incoming_commands(
        &self,
        device: BTreeSet<MacAddr8>,
        channel_size: usize,
    ) -> impl Future<Output = Result<Receiver<Event>, Error>>;
}

impl NetworkManager for Sender<Message> {
    async fn get_ieee_address_from_short_id(
        &self,
        short_id: u16,
    ) -> Result<Option<MacAddr8>, Error> {
        let (response, result) = channel();
        self.send(Message::GetIeeeAddressFromShortId { short_id, response })
            .await?;
        Ok(result.await?)
    }

    async fn get_short_id_from_ieee_address(
        &self,
        ieee_address: MacAddr8,
    ) -> Result<Option<u16>, Error> {
        let (response, result) = channel();
        self.send(Message::GetShortIdFromIeeeAddress {
            ieee_address,
            response,
        })
        .await?;
        Ok(result.await?)
    }

    async fn state(&self) -> Result<BTreeMap<MacAddr8, Device>, Error> {
        let (response, result) = channel();
        self.send(Message::GetDevices { response }).await?;
        Ok(result.await?)
    }

    async fn subscribe_to_incoming_commands(
        &self,
        device: BTreeSet<MacAddr8>,
        channel_size: usize,
    ) -> Result<Receiver<Event>, Error> {
        let (sender, receiver) = tokio::sync::mpsc::channel(channel_size);
        self.send(Message::SubscribeToIncomingCommands {
            devices: device,
            sender,
        })
        .await?;
        Ok(receiver)
    }
}

impl NetworkManager for Coordinator {
    async fn get_ieee_address_from_short_id(
        &self,
        short_id: u16,
    ) -> Result<Option<MacAddr8>, Error> {
        self.network_manager
            .get_ieee_address_from_short_id(short_id)
            .await
    }

    async fn get_short_id_from_ieee_address(
        &self,
        ieee_address: MacAddr8,
    ) -> Result<Option<u16>, Error> {
        self.network_manager
            .get_short_id_from_ieee_address(ieee_address)
            .await
    }

    async fn state(&self) -> Result<BTreeMap<MacAddr8, Device>, Error> {
        self.network_manager.state().await
    }

    async fn subscribe_to_incoming_commands(
        &self,
        device: BTreeSet<MacAddr8>,
        channel_size: usize,
    ) -> Result<Receiver<Event>, Error> {
        self.network_manager
            .subscribe_to_incoming_commands(device, channel_size)
            .await
    }
}
