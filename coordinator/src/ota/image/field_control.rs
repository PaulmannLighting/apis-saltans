use bitflags::bitflags;

const SECURITY_CREDENTIAL_VERSION_LENGTH: usize = 1;
const UPGRADE_FILE_DESTINATION_LENGTH: usize = 8;
const HARDWARE_VERSIONS_LENGTH: usize = 4;

/// Optional fields present in a Zigbee OTA image header.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FieldControl(u16);

bitflags! {
    impl FieldControl: u16 {
        /// A security credential version follows the base header.
        const SECURITY_CREDENTIAL_VERSION = 0x0001;
        /// An IEEE upgrade-file destination follows the base header.
        const UPGRADE_FILE_DESTINATION = 0x0002;
        /// Minimum and maximum hardware versions follow the base header.
        const HARDWARE_VERSIONS = 0x0004;
    }
}

impl FieldControl {
    pub(super) const fn optional_header_length(self) -> usize {
        self.optional_length(
            Self::SECURITY_CREDENTIAL_VERSION,
            SECURITY_CREDENTIAL_VERSION_LENGTH,
        ) + self.optional_length(
            Self::UPGRADE_FILE_DESTINATION,
            UPGRADE_FILE_DESTINATION_LENGTH,
        ) + self.optional_length(Self::HARDWARE_VERSIONS, HARDWARE_VERSIONS_LENGTH)
    }

    const fn optional_length(self, flag: Self, length: usize) -> usize {
        if self.contains(flag) { length } else { 0 }
    }
}
