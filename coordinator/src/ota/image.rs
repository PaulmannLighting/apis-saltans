//! Validated Zigbee OTA upgrade image files.

use std::fmt::{self, Debug, Formatter};

pub use self::error::ParseImageError;
pub use self::field_control::FieldControl;
pub use self::header::{BaseHeaderBytes, Header, HeaderString};
pub use self::parser::ParseImage;
use self::source::ImageSource;
pub(super) use self::transfer::ImageTransfer;

mod error;
mod field_control;
mod header;
mod parser;
mod source;
mod transfer;

#[cfg(test)]
mod tests;

/// A validated Zigbee OTA image backed by an owned seekable reader.
pub struct Image {
    header: Header,
    source: Box<dyn ImageSource>,
}

impl Debug for Image {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Image")
            .field("header", &self.header)
            .finish_non_exhaustive()
    }
}

impl Image {
    const fn new(header: Header, source: Box<dyn ImageSource>) -> Self {
        Self { header, source }
    }

    /// Return the parsed OTA image header.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Return the manufacturer, image type, and file version from the image header.
    #[must_use]
    pub const fn id(&self) -> zb_zcl::ota_upgrade::ImageId {
        self.header.id()
    }

    /// Return the complete image size, including the OTA header.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.header.image_length()
    }

    /// Return whether the image contains no bytes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Return the OTA header length in bytes.
    #[must_use]
    pub const fn header_length(&self) -> u16 {
        self.header.header_length()
    }

    /// Return the Zigbee stack version from the OTA header.
    #[must_use]
    pub const fn zigbee_stack_version(&self) -> u16 {
        self.header.zigbee_stack_version()
    }

    /// Return the human-readable OTA header string before its null terminator.
    #[must_use]
    pub const fn header_string(&self) -> &HeaderString {
        self.header.header_string()
    }

    /// Return the optional security credential version.
    #[must_use]
    pub const fn security_credential_version(&self) -> Option<u8> {
        self.header.security_credential_version()
    }

    /// Return the optional IEEE address to which this file is restricted.
    #[must_use]
    pub const fn upgrade_file_destination(&self) -> Option<zb_core::IeeeAddress> {
        self.header.upgrade_file_destination()
    }

    /// Return the optional inclusive hardware-version range.
    #[must_use]
    pub const fn hardware_versions(&self) -> Option<(u16, u16)> {
        self.header.hardware_versions()
    }

    pub(super) fn into_transfer(self) -> ImageTransfer {
        let Self { header, source } = self;
        ImageTransfer::spawn(header, source)
    }
}
