use std::collections::BTreeSet;

use either::Either;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;
use zb_core::{FullAddress, IeeeAddress, short_id};

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
        short_id: short_id::Device,
    ) -> impl Future<Output = Result<Option<IeeeAddress>, Error>> + Send;

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
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<Option<short_id::Device>, Error>> + Send;

    /// Resolve the given IEEE address into an [`Address`].
    ///
    /// # Returns
    ///
    /// Returns `Some(address)` if the address is known, `None` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the actor failed.
    fn get_full_address(
        &self,
        address: Either<IeeeAddress, short_id::Device>,
    ) -> impl Future<Output = Result<Option<FullAddress>, Error>> + Send
    where
        Self: Sync,
    {
        async move {
            match address {
                Either::Left(ieee_address) => self
                    .get_short_id_from_ieee_address(ieee_address)
                    .await
                    .map(|option| option.map(|short_id| FullAddress::new(ieee_address, short_id))),
                Either::Right(short_id) => {
                    self.get_ieee_address_from_short_id(short_id)
                        .await
                        .map(|option| {
                            option.map(|ieee_address| FullAddress::new(ieee_address, short_id))
                        })
                }
            }
        }
    }

    /// Subscribes to a stream of incoming commands.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication with the actor failed.
    fn subscribe_to_incoming_commands(
        &self,
        device: BTreeSet<IeeeAddress>,
        channel_size: usize,
    ) -> impl Future<Output = Result<Receiver<Event>, Error>> + Send;

    /// Yield devices of the network.
    fn devices(&self) -> impl Future<Output = Result<Box<[Device]>, Error>> + Send;
}

impl NetworkManager for Sender<Message> {
    async fn get_ieee_address_from_short_id(
        &self,
        short_id: short_id::Device,
    ) -> Result<Option<IeeeAddress>, Error> {
        let (response, result) = channel();
        self.send(Message::GetIeeeAddressFromShortId { short_id, response })
            .await?;
        Ok(result.await?)
    }

    async fn get_short_id_from_ieee_address(
        &self,
        ieee_address: IeeeAddress,
    ) -> Result<Option<short_id::Device>, Error> {
        let (response, result) = channel();
        self.send(Message::GetShortIdFromIeeeAddress {
            ieee_address,
            response,
        })
        .await?;
        Ok(result.await?)
    }

    async fn subscribe_to_incoming_commands(
        &self,
        device: BTreeSet<IeeeAddress>,
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

    async fn devices(&self) -> Result<Box<[Device]>, Error> {
        let (tx, rx) = channel();
        self.send(Message::GetDevices(tx)).await?;
        Ok(rx.await?)
    }
}

impl NetworkManager for Coordinator {
    async fn get_ieee_address_from_short_id(
        &self,
        short_id: short_id::Device,
    ) -> Result<Option<IeeeAddress>, Error> {
        self.network_manager
            .get_ieee_address_from_short_id(short_id)
            .await
    }

    async fn get_short_id_from_ieee_address(
        &self,
        ieee_address: IeeeAddress,
    ) -> Result<Option<short_id::Device>, Error> {
        self.network_manager
            .get_short_id_from_ieee_address(ieee_address)
            .await
    }

    async fn subscribe_to_incoming_commands(
        &self,
        device: BTreeSet<IeeeAddress>,
        channel_size: usize,
    ) -> Result<Receiver<Event>, Error> {
        self.network_manager
            .subscribe_to_incoming_commands(device, channel_size)
            .await
    }

    async fn devices(&self) -> Result<Box<[Device]>, Error> {
        self.network_manager.devices().await
    }
}
