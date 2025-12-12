use std::fmt::Display;

use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

use super::device_type::DeviceType;

/// MAC Capability Flags as defined in the IEEE 802.15.4 standard.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromLeStream, ToLeStream)]
pub struct MacCapabilityFlags(u8);

bitflags! {
    impl MacCapabilityFlags: u8 {
        /// Indicates whether the node is capable of becoming a PAN coordinator.
        const ALTERNATE_PAN_COORDINATOR = 0b1000_0000;
        /// Indicates the node is a full-function device (FFD) or reduced-function device (RFD).
        const DEVICE_TYPE = 0b0100_0000;
        /// Indicates the current power source of the node.
        const POWER_SOURCE = 0b0010_0000;
        /// Indicates whether the receiver is on when the device is idle.
        const RECEIVER_ON_WHEN_IDLE = 0b0001_0000;
        /// Indicates whether the node is capable of sending and receiving frames secured using the security suite.
        const SECURITY_CAPABLE = 0b0000_0010;
        /// Indicates whether the recipient shall allocate a network address for the node.
        const ALLOCATE_ADDRESS = 0b0000_0001;
    }
}

impl MacCapabilityFlags {
    /// Returns whether the node is capable of becoming a PAN coordinator.
    #[must_use]
    pub const fn alternate_pan_coordinator(self) -> bool {
        self.contains(Self::ALTERNATE_PAN_COORDINATOR)
    }

    /// Returns the device type.
    #[must_use]
    pub const fn device_type(self) -> DeviceType {
        if self.contains(Self::DEVICE_TYPE) {
            DeviceType::FullFunctionDevice
        } else {
            DeviceType::ReducedFunctionDevice
        }
    }

    /// Returns whether the current power source is mains power.
    #[must_use]
    pub const fn is_mains_power(self) -> bool {
        self.contains(Self::POWER_SOURCE)
    }

    /// Returns whether the receiver is on when idle.
    #[must_use]
    pub const fn is_receiver_on_when_idle(self) -> bool {
        self.contains(Self::RECEIVER_ON_WHEN_IDLE)
    }

    /// Returns whether the node is capable of sending and
    /// receiving frames secured using the security suite.
    #[must_use]
    pub const fn is_security_capable(self) -> bool {
        self.contains(Self::SECURITY_CAPABLE)
    }

    /// Returns whether the node is capable of allocating addresses.
    #[must_use]
    pub const fn allocate_address(self) -> bool {
        self.contains(Self::ALLOCATE_ADDRESS)
    }
}

impl Display for MacCapabilityFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut names = self.iter_names();

        if let Some((name, flag)) = names.next() {
            write!(f, "{name} ({flag:#04X})")?;

            for (name, flag) in names {
                write!(f, ", {name} ({flag:#04X})")?;
            }
        }

        write!(f, "]")
    }
}
