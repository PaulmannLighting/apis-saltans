//! Generic API to implement storage of the Zigbee network state.

use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::oneshot::channel;
use zb_core::{IeeeAddress, short_id};

pub use self::error::Error;
pub use self::message::Message;
use crate::Device;

mod error;
mod message;

/// The storage server handle.
pub type Server = Receiver<Message>;

/// Storage client handle trait.
pub trait Storage {
    /// Return the current devices from the storage.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if loading from the storage fails.
    fn devices(&self) -> impl Future<Output = Result<Box<[Device]>, Error>> + Send;

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
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<Option<Device>, Error>> + Send;

    /// Return a device from the storage given its short ID.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if querying to the storage fails.
    fn get_by_short_id(
        &self,
        short_id: short_id::Device,
    ) -> impl Future<Output = Result<Option<Device>, Error>> + Send;

    /// Return a device from the storage given its IEEE address.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if querying to the storage fails.
    fn get_by_ieee_address(
        &self,
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<Option<Device>, Error>> + Send;

    fn get_short_id(
        &self,
        ieee_address: IeeeAddress,
    ) -> impl Future<Output = Result<Option<short_id::Device>, Error>> + Send;

    fn get_ieee_address(
        &self,
        short_id: short_id::Device,
    ) -> impl Future<Output = Result<Option<IeeeAddress>, Error>> + Send;

    fn update_short_id(
        &self,
        ieee_address: IeeeAddress,
        short_id: short_id::Device,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Storage for Sender<Message> {
    async fn devices(&self) -> Result<Box<[Device]>, Error> {
        let (tx, rx) = channel();
        self.send(Message::Devices(tx)).await?;
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

    async fn remove(&self, ieee_address: IeeeAddress) -> Result<Option<Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::Remove {
            ieee_address,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn get_by_short_id(&self, short_id: short_id::Device) -> Result<Option<Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::GetByShortId {
            short_id,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn get_by_ieee_address(
        &self,
        ieee_address: IeeeAddress,
    ) -> Result<Option<Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::GetByIeeeAddress {
            ieee_address,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn get_short_id(
        &self,
        ieee_address: IeeeAddress,
    ) -> Result<Option<short_id::Device>, Error> {
        let (tx, rx) = channel();
        self.send(Message::GetShortId {
            ieee_address,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn get_ieee_address(
        &self,
        short_id: short_id::Device,
    ) -> Result<Option<IeeeAddress>, Error> {
        let (tx, rx) = channel();
        self.send(Message::GetIeeeAddress {
            short_id,
            response: tx,
        })
        .await?;
        Ok(rx.await??)
    }

    async fn update_short_id(
        &self,
        ieee_address: IeeeAddress,
        short_id: short_id::Device,
    ) -> Result<(), Error> {
        self.send(Message::UpdateShortId {
            ieee_address,
            short_id,
        })
        .await?;
        Ok(())
    }
}
