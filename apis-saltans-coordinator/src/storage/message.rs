use std::io::Result;

use apis_saltans_core::{Address, IeeeAddress};
use tokio::sync::oneshot::Sender;

use crate::Device;

/// Messages exchanged with the storage manager.
#[derive(Debug)]
pub enum Message {
    /// Load the network state.
    Devices(Sender<Result<Box<[Device]>>>),

    /// Add a device.
    Add {
        /// The device to add.
        device: Device,
        /// The response channel.
        ///
        /// Returns the previously stored device on ID clashes.
        response: Sender<Result<Option<Device>>>,
    },

    /// Remove a device by its address.
    Remove {
        /// The address of the device to remove.
        address: Address,
        /// The response channel.
        ///
        /// Returns the removed device, if any.
        response: Sender<Result<Option<Device>>>,
    },

    /// Return a device given its short ID.
    GetByShortId {
        /// The short ID of the device.
        short_id: u16,
        /// The response channel.
        response: Sender<Result<Option<Device>>>,
    },

    /// Return a device given its IEEE address.
    GetByIeeeAddress {
        /// The IEEE address of the device.
        ieee_address: IeeeAddress,
        /// The response channel.
        response: Sender<Result<Option<Device>>>,
    },

    GetShortId {
        ieee_address: IeeeAddress,
        response: Sender<Result<Option<u16>>>,
    },

    GetIeeeAddress {
        short_id: u16,
        response: Sender<Result<Option<IeeeAddress>>>,
    },

    UpdateShortId {
        ieee_address: IeeeAddress,
        short_id: u16,
    },
}
