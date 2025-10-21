use bitflags::bitflags;

use super::device_type::DeviceType;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct MacCapabilityFlags(u8);

bitflags! {
    impl MacCapabilityFlags: u8 {
        const ALTERNATE_PAN_COORDINATOR = 0b1000_0000;
        const DEVICE_TYPE = 0b0100_0000;
        const POWER_SOURCE = 0b0010_0000;
        const RECEIVER_ON_WHEN_IDLE = 0b0001_0000;
        const RESERVED = 0b0000_1100;
        const SECURITY_CAPABLE = 0b0000_0010;
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

    /// Returns the reserved fields value.
    #[must_use]
    pub fn reserved(self) -> u8 {
        (self & Self::RESERVED).bits() >> 2
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
