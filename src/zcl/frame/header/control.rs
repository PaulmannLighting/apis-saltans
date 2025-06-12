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
    }
}

impl Control {
    /// Creates a new `Control` instance with the specified flags.
    #[must_use]
    pub fn new(
        typ: Type,
        manufacturer_specific: bool,
        direction: Direction,
        disable_client_response: bool,
    ) -> Self {
        let mut flags = Self((typ as u8) << Self::TYPE.bits().trailing_zeros());

        if manufacturer_specific {
            flags.set(Self::MANUFACTURER_SPECIFIC, true);
        }

        flags.set(Self::DIRECTION, direction == Direction::ServerToClient);

        if disable_client_response {
            flags.set(Self::DISABLE_DEFAULT_RESPONSE, true);
        }

        flags
    }

    /// Return the command type.
    ///
    /// # Errors
    ///
    /// If the command type is not recognized, it returns an error with the raw value.
    pub fn typ(self) -> Result<Type, u8> {
        Type::try_from((self.0 & Self::TYPE.bits()) >> Self::TYPE.bits().trailing_zeros())
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
    use super::{Control, Direction, Type};

    #[test]
    fn test_type_trailing_zeros() {
        assert_eq!(Control::TYPE.bits().trailing_zeros(), 6);
    }

    #[test]
    fn test_cluster_specific_flag() {
        assert_eq!(
            (Type::ClusterSpecific as u8) << Control::TYPE.bits().trailing_zeros(),
            0b0100_0000
        );
    }

    #[test]
    fn test_global_flag() {
        assert_eq!(
            (Type::Global as u8) << Control::TYPE.bits().trailing_zeros(),
            0b0000_0000
        );
    }

    #[test]
    fn test_control_cluster_specific() {
        let control = Control::new(Type::ClusterSpecific, true, Direction::ServerToClient, true);
        assert_eq!(control.typ(), Ok(Type::ClusterSpecific));
        assert!(control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ServerToClient);
        assert!(control.disable_default_response());
    }

    #[test]
    fn test_control_global() {
        let control = Control::new(Type::Global, true, Direction::ServerToClient, true);
        assert_eq!(control.typ(), Ok(Type::Global));
        assert!(control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ServerToClient);
        assert!(control.disable_default_response());
    }

    #[test]
    fn test_control_manufacturer_unspecific() {
        let control = Control::new(
            Type::ClusterSpecific,
            false,
            Direction::ServerToClient,
            true,
        );
        assert_eq!(control.typ(), Ok(Type::ClusterSpecific));
        assert!(!control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ServerToClient);
        assert!(control.disable_default_response());
    }

    #[test]
    fn test_control_disable_client_to_server() {
        let control = Control::new(Type::ClusterSpecific, true, Direction::ClientToServer, true);
        assert_eq!(control.typ(), Ok(Type::ClusterSpecific));
        assert!(control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ClientToServer);
        assert!(control.disable_default_response());
    }

    #[test]
    fn test_control_enable_client_response() {
        let control = Control::new(
            Type::ClusterSpecific,
            true,
            Direction::ClientToServer,
            false,
        );
        assert_eq!(control.typ(), Ok(Type::ClusterSpecific));
        assert!(control.is_manufacturer_specific());
        assert_eq!(control.direction(), Direction::ClientToServer);
        assert!(!control.disable_default_response());
    }
}
