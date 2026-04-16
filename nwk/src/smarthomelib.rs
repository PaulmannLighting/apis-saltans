//! Smarthomelib API implementations.

#![cfg(feature = "smarthomelib")]

use zigbee::Endpoint;

pub use self::color_control::Error as ColorControlError;

mod color_control;
mod event;
mod executor;
mod network_manager;
mod on_off;

/// A Zigbee source.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Source {
    pan_id: u16,
    endpoint: Endpoint,
}

impl Source {
    /// Crate a new Zigbee source.
    #[must_use]
    pub const fn new(pan_id: u16, endpoint: Endpoint) -> Self {
        Self { pan_id, endpoint }
    }

    /// Return the PAN ID.
    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    /// Return the endpoint.
    #[must_use]
    pub const fn endpoint(&self) -> Endpoint {
        self.endpoint
    }
}
