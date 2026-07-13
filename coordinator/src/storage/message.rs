use std::io::Result;

use tokio::sync::oneshot::Sender;
use zb_core::{IeeeAddress, short_id};

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
        ieee_address: IeeeAddress,
        /// The response channel.
        ///
        /// Returns the removed device, if any.
        response: Sender<Result<Option<Device>>>,
    },

    /// Return a device given its short ID.
    GetByShortId {
        /// The short ID of the device.
        short_id: short_id::Device,
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

    /// Return the short ID associated with an IEEE address.
    GetShortId {
        /// The IEEE address to look up.
        ieee_address: IeeeAddress,
        /// The response channel.
        response: Sender<Result<Option<short_id::Device>>>,
    },

    /// Return the IEEE address associated with a short ID.
    GetIeeeAddress {
        /// The short ID to look up.
        short_id: short_id::Device,
        /// The response channel.
        response: Sender<Result<Option<IeeeAddress>>>,
    },

    /// Update the short ID associated with an IEEE address.
    UpdateShortId {
        /// The IEEE address to update.
        ieee_address: IeeeAddress,
        /// The new short ID.
        short_id: short_id::Device,

        /// The result of the command.
        response: Sender<Result<()>>,
    },
}
