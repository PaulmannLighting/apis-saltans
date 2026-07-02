//! Generic API to implement storage of the Zigbee network state.

use apis_saltans_core::Address;
use macaddr::MacAddr8;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;

pub use self::error::Error;
pub use self::message::Message;
use crate::{Device, State};

mod error;
mod message;

/// The storage server handle.
pub type Server = Receiver<Message>;

/// Storage client handle trait.
pub trait Storage {
    /// Load the current state from the storage.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if loading from the storage fails.
    fn load(&self) -> impl Future<Output = Result<Option<State>, Error>> + Send;

    /// Save the current state.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if writing to the storage fails.
    fn save(&self, state: State) -> impl Future<Output = Result<(), Error>> + Send;

    /// Add a device to the storage.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if writing to the storage fails.
    fn add(&self, device: Device) -> impl Future<Output = Result<Option<Device>, Error>> + Send;

    /// Removes a device from the storage.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if writing to the storage fails.
    fn remove(
        &self,
        address: Address,
    ) -> impl Future<Output = Result<Option<Device>, Error>> + Send;

    /// Return a device from the storage given its full address.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if querying to the storage fails.
    fn get_by_address(
        &self,
        address: Address,
    ) -> impl Future<Output = Result<Option<Device>, Error>> + Send;

    /// Return a device from the storage given its IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if querying to the storage fails.
    fn get_by_ieee_address(
        &self,
        ieee_address: MacAddr8,
    ) -> impl Future<Output = Result<Option<Device>, Error>> + Send;

    /// Return a device from the storage given its short ID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if querying to the storage fails.
    fn get_by_short_id(
        &self,
        short_id: u16,
    ) -> impl Future<Output = Result<Option<Device>, Error>> + Send;
}

impl Storage for Sender<Message> {
    async fn load(&self) -> Result<Option<State>, Error> {
        let (tx, rx) = channel();
        self.send(Message::Load(tx)).await?;
        Ok(rx.await??)
    }

    async fn save(&self, state: State) -> Result<(), Error> {
        let (tx, rx) = channel();
        self.send(Message::Save {
            state,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn add(&self, device: Device) -> Result<Option<Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::Add {
            device,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn remove(&self, address: Address) -> Result<Option<Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::Remove {
            address,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn get_by_address(&self, address: Address) -> Result<Option<Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::GetByAddress {
            address,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn get_by_ieee_address(&self, ieee_address: MacAddr8) -> Result<Option<Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::GetByIeeeAddress {
            ieee_address,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn get_by_short_id(&self, short_id: u16) -> Result<Option<Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::GetByShortId {
            short_id,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }
}
