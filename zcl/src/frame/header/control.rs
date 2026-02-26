use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};
use zigbee::Direction;

use super::scope::Scope;

/// ZCL frame control flags.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct Control(u8);

bitflags! {
    impl Control: u8 {
        /// The command type.
        const TYPE = 0b000_00011;
        /// The command type is manufacturer specific.
        const MANUFACTURER_SPECIFIC = 0b0000_0100;
        /// The direction of the command.
        const DIRECTION = 0b0000_1000;
        /// The default response is disabled.
        const DISABLE_DEFAULT_RESPONSE = 0b0001_0000;
    }
}

impl Control {
    /// Creates a new `Control` instance with the specified flags.
    #[must_use]
    pub fn new(
        typ: Scope,
        manufacturer_specific: bool,
        direction: Direction,
        disable_default_response: bool,
    ) -> Self {
        let mut flags = Self(typ as u8);

        if manufacturer_specific {
            flags.insert(Self::MANUFACTURER_SPECIFIC);
        }

        match direction {
            Direction::ClientToServer => {
                flags.remove(Self::DIRECTION);
            }
            Direction::ServerToClient => {
                flags.insert(Self::DIRECTION);
            }
        }

        if disable_default_response {
            flags.insert(Self::DISABLE_DEFAULT_RESPONSE);
        }

        flags
    }

    /// Return the command type.
    ///
    /// # Errors
    ///
    /// If the command type is not recognized, it returns an error with the raw value.
    pub fn typ(self) -> Result<Scope, u8> {
        Scope::try_from(self.0 & Self::TYPE.bits())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_specific() {
        let control = Control::new(
            Scope::ClusterSpecific,
            true,
            Direction::ServerToClient,
            true,
        );
        assert_eq!(control.typ(), Ok(Scope::ClusterSpecific));
        assert!(control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ServerToClient);
        assert!(control.disable_default_response());
    }

    #[test]
    fn global() {
        let control = Control::new(Scope::Global, true, Direction::ServerToClient, true);
        assert_eq!(control.typ(), Ok(Scope::Global));
        assert!(control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ServerToClient);
        assert!(control.disable_default_response());
    }

    #[test]
    fn manufacturer_unspecific() {
        let control = Control::new(
            Scope::ClusterSpecific,
            false,
            Direction::ServerToClient,
            true,
        );
        assert_eq!(control.typ(), Ok(Scope::ClusterSpecific));
        assert!(!control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ServerToClient);
        assert!(control.disable_default_response());
    }

    #[test]
    fn disable_client_to_server() {
        let control = Control::new(
            Scope::ClusterSpecific,
            true,
            Direction::ClientToServer,
            true,
        );
        assert_eq!(control.typ(), Ok(Scope::ClusterSpecific));
        assert!(control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ClientToServer);
        assert!(control.disable_default_response());
    }

    #[test]
    fn enable_client_response() {
        let control = Control::new(
            Scope::ClusterSpecific,
            true,
            Direction::ClientToServer,
            false,
        );
        assert_eq!(control.typ(), Ok(Scope::ClusterSpecific));
        assert!(control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ClientToServer);
        assert!(!control.disable_default_response());
    }
}
