//! A library for prototyping Zigbee coordinator devices.

use std::io;

/// A Zigbee coordinator device.
pub trait Coordinator {
    /// Initializes the coordinator device.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if initialization fails.
    fn initialize(&mut self) -> io::Result<()>;

    /// Forms a new Zigbee network with the specified PAN ID and channel.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if network formation fails.
    fn form_network(&mut self, pan_id: u16, channel: u8) -> io::Result<()>;
}
