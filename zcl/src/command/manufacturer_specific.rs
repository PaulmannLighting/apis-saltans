use zigbee::Direction;

use crate::{Command, Scope};

/// A manufacturer-specific command.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ManufacturerSpecific<T> {
    manufacturer_code: u16,
    payload: T,
}

impl<T> ManufacturerSpecific<T> {
    /// Create a new manufacturer-specific command.
    #[must_use]
    pub const fn new(manufacturer_code: u16, payload: T) -> Self {
        Self {
            manufacturer_code,
            payload,
        }
    }

    /// Return the manufacturer code.
    pub const fn manufacturer_code(&self) -> u16 {
        self.manufacturer_code
    }

    /// Return the inner payload.
    pub fn into_payload(self) -> T {
        self.payload
    }
}

impl<T> Command for ManufacturerSpecific<T>
where
    T: Command,
{
    const ID: u8 = T::ID;
    const DIRECTION: Direction = T::DIRECTION;
    const SCOPE: Scope = T::SCOPE;
    const DISABLE_DEFAULT_RESPONSE: bool = T::DISABLE_DEFAULT_RESPONSE;

    fn manufacturer_code(&self) -> Option<u16> {
        Some(self.manufacturer_code)
    }
}
