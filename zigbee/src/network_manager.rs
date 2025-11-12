//! A Zigbee network manager.

use std::io::Read;

/// Trait to manage Zigbee networks.
///
/// TODO: Implement appropriate methods to manage Zigbee networks and access clusters.
pub trait NetworkManager: Sized {
    /// Configuration used for initializing the network manager.
    type Configuration;

    /// Loads a network manager config from a source.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if loading fails.
    fn load<T>(source: T) -> std::io::Result<Self>
    where
        T: Read;

    /// Initializes the network manager.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if initialization fails.
    fn init(&mut self, configuration: Self::Configuration) -> std::io::Result<()>;

    /// Resets the network manager.
    ///
    /// # Errors
    ///
    /// Returns an [Error](std::io::Error) if resetting fails.
    fn reset(&mut self) -> std::io::Result<()>;
}
