use le_stream::{FromLeStream, ToLeStream};

pub use self::device_type::DeviceType;
pub use self::flags::Flags;
pub use self::frequency_band::FrequencyBand;
pub use self::logical_type::LogicalType;
pub use self::mac_capability_flags::MacCapabilityFlags;
pub use self::server_mask::ServerMask;

mod device_type;
mod flags;
mod frequency_band;
mod logical_type;
mod mac_capability_flags;
mod server_mask;

/// Node Descriptor.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, FromLeStream, ToLeStream)]
pub struct Descriptor {
    flags: Flags,
    mac_capability_flags: MacCapabilityFlags,
    manufacturer_code: u16,
    maximum_buffer_size: u8,
    maximum_incoming_transfer_size: u16,
    server_mask: ServerMask,
    maximum_outgoing_transfer_size: u16,
    capability: u8,
}

impl Descriptor {
    /// Creates a new `Descriptor`.
    #[must_use]
    pub const fn new(
        flags: Flags,
        mac_capability_flags: MacCapabilityFlags,
        manufacturer_code: u16,
        maximum_buffer_size: u8,
        maximum_incoming_transfer_size: u16,
        server_mask: ServerMask,
        maximum_outgoing_transfer_size: u16,
    ) -> Self {
        Self {
            flags,
            mac_capability_flags,
            manufacturer_code,
            maximum_buffer_size,
            maximum_incoming_transfer_size,
            server_mask,
            maximum_outgoing_transfer_size,
            capability: 0x00, // Currently unused as per Zigbee spec.
        }
    }

    /// Returns the flags.
    #[must_use]
    pub const fn flags(&self) -> &Flags {
        &self.flags
    }

    /// Returns the MAC capability flags.
    #[must_use]
    pub const fn mac_capability_flags(&self) -> &MacCapabilityFlags {
        &self.mac_capability_flags
    }

    /// Returns the manufacturer code.
    #[must_use]
    pub const fn manufacturer_code(&self) -> u16 {
        self.manufacturer_code
    }

    /// Returns the maximum buffer size.
    #[must_use]
    pub const fn maximum_buffer_size(&self) -> u8 {
        self.maximum_buffer_size
    }

    /// Returns the maximum incoming transfer size.
    #[must_use]
    pub const fn maximum_incoming_transfer_size(&self) -> u16 {
        self.maximum_incoming_transfer_size
    }

    /// Returns the server mask.
    #[must_use]
    pub const fn server_mask(&self) -> &ServerMask {
        &self.server_mask
    }

    /// Returns the maximum outgoing transfer size.
    #[must_use]
    pub const fn maximum_outgoing_transfer_size(&self) -> u16 {
        self.maximum_outgoing_transfer_size
    }

    /// Returns the capability.
    #[must_use]
    pub const fn capability(&self) -> u8 {
        self.capability
    }
}
