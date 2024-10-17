//! A Zigbee network manager.

use std::io::Read;

pub trait NetworkManager {
    /// Configuration used for initializing the network manager.
    type Configuration;

    /// Loads a network manager config from a source.
    fn load<T>(source: T) -> std::io::Result<Self>
    where
        T: Read;

    /// Initializes the network manager.
    fn init(&mut self, configuration: Self::Configuration) -> std::io::Result<()>;

    /// Resets the network manager.
    fn reset(&mut self) -> std::io::Result<()>;
}
