use bitflags::bitflags;

use super::direction::Direction;
use super::typ::Type;

/// ZCL frame control flags.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Control(u8);

bitflags! {
    impl Control: u8 {
        const TYPE = 0b1100_0000;
        const MANUFACTURER_SPECIFIC = 0b0010_0000;
        const DIRECTION = 0b0001_0000;
        const DISABLE_DEFAULT_RESPONSE = 0b0000_1000;
        const RESERVED = 0b0000_0111;
    }
}

impl Control {
    /// Creates a new `Control` instance with the specified flags.
    #[must_use]
    pub const fn new(flags: u8) -> Self {
        Self(flags)
    }

    /// Return the command type.
    pub const fn typ(self) -> Result<Type, u8> {
        match self.0 & Self::TYPE.bits() {
            0x00 => Ok(Type::Global),
            0x01 => Ok(Type::ClusterSpecific),
            other => Err(other),
        }
    }

    /// Return whether the command is manufacturer specific.
    #[must_use]
    pub const fn is_manufacturer_specific(self) -> bool {
        self.contains(Self::MANUFACTURER_SPECIFIC)
    }

    /// Return the direction of the command.
    #[must_use]
    pub const fn direction(self) -> Direction {
        if self.contains(Self::DIRECTION) {
            Direction::ServerToClient
        } else {
            Direction::ClientToServer
        }
    }

    /// Return whether the default response is disabled.
    #[must_use]
    pub const fn disable_default_response(self) -> bool {
        self.contains(Self::DISABLE_DEFAULT_RESPONSE)
    }
}
