use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

use super::device_type::DeviceType;

/// MAC Capability Flags as defined in the IEEE 802.15.4 standard.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MacCapabilityFlags(u8);

bitflags! {
    impl MacCapabilityFlags: u8 {
        /// Indicates whether the node is capable of becoming a PAN coordinator.
        const ALTERNATE_PAN_COORDINATOR = 0b0000_0001;
        /// Indicates the node is a full-function device (FFD) or reduced-function device (RFD).
        const DEVICE_TYPE = 0b0000_0010;
        /// Indicates the current power source of the node.
        const POWER_SOURCE = 0b0000_0100;
        /// Indicates whether the receiver is on when the device is idle.
        const RECEIVER_ON_WHEN_IDLE = 0b0000_1000;
        /// Indicates whether the node is capable of sending and receiving frames secured using the security suite.
        const SECURITY_CAPABLE = 0b0100_0000;
        /// Indicates whether the recipient shall allocate a network address for the node.
        const ALLOCATE_ADDRESS = 0b1000_0000;
    }
}

impl_bitflags_display_and_from_str!(MacCapabilityFlags);

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

    /// Sets the device type.
    pub fn set_device_type(&mut self, device_type: DeviceType) {
        match device_type {
            DeviceType::FullFunctionDevice => {
                self.insert(Self::DEVICE_TYPE);
            }
            DeviceType::ReducedFunctionDevice => {
                self.remove(Self::DEVICE_TYPE);
            }
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

    /// Returns whether the recipient shall allocate a network address for the node.
    #[must_use]
    pub const fn allocate_address(self) -> bool {
        self.contains(Self::ALLOCATE_ADDRESS)
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};

    use super::MacCapabilityFlags;

    #[test]
    fn flags_when_read_then_match_zigbee_bit_assignments() {
        assert_eq!(MacCapabilityFlags::ALTERNATE_PAN_COORDINATOR.bits(), 0x01);
        assert_eq!(MacCapabilityFlags::DEVICE_TYPE.bits(), 0x02);
        assert_eq!(MacCapabilityFlags::POWER_SOURCE.bits(), 0x04);
        assert_eq!(MacCapabilityFlags::RECEIVER_ON_WHEN_IDLE.bits(), 0x08);
        assert_eq!(MacCapabilityFlags::SECURITY_CAPABLE.bits(), 0x40);
        assert_eq!(MacCapabilityFlags::ALLOCATE_ADDRESS.bits(), 0x80);
    }

    #[test]
    fn flags_when_serialized_then_match_zigbee_wire_layout() {
        let flags = MacCapabilityFlags::DEVICE_TYPE
            | MacCapabilityFlags::POWER_SOURCE
            | MacCapabilityFlags::RECEIVER_ON_WHEN_IDLE
            | MacCapabilityFlags::ALLOCATE_ADDRESS;

        let mut bytes = flags.to_le_stream();
        assert_eq!(bytes.next(), Some(0x8E));
        assert_eq!(bytes.next(), None);
        assert_eq!(
            MacCapabilityFlags::from_le_stream([0x8E].into_iter()),
            Some(flags)
        );
    }
}
