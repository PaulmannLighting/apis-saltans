use std::io;

use thiserror::Error;

/// Error returned while parsing a Zigbee OTA upgrade image.
#[derive(Debug, Error)]
pub enum ParseImageError {
    /// Reading or seeking the OTA image source failed.
    #[error("failed to read OTA image: {0}")]
    Io(#[from] io::Error),
    /// The file does not contain the complete fixed OTA header.
    #[error("the OTA image header is truncated")]
    TruncatedHeader,
    /// The OTA file identifier is invalid.
    #[error("invalid OTA file identifier {0:#010x}")]
    InvalidFileIdentifier(u32),
    /// The OTA header version is not supported.
    #[error("unsupported OTA header version {0:#06x}")]
    UnsupportedHeaderVersion(u16),
    /// The field-control value includes unsupported optional fields.
    #[error("unsupported OTA header field-control bits {0:#06x}")]
    UnsupportedFieldControl(u16),
    /// The fixed-width header string is not null-terminated ASCII.
    #[error("the OTA header string is not null-terminated ASCII")]
    InvalidHeaderString,
    /// The declared header length does not match the optional fields.
    #[error("invalid OTA header length {declared}; expected {expected}")]
    InvalidHeaderLength {
        /// Length declared in the OTA header.
        declared: u16,
        /// Length implied by the field-control bits.
        expected: usize,
    },
    /// The byte source is too large for the OTA file-size field or this platform.
    #[error("OTA image is larger than the supported file size")]
    ImageTooLarge,
    /// The declared total size differs from the supplied file length.
    #[error("invalid OTA image size {declared}; actual size is {actual}")]
    InvalidImageSize {
        /// Size declared in the OTA header.
        declared: u32,
        /// Number of supplied bytes.
        actual: u32,
    },
    /// The optional hardware-version range is reversed.
    #[error("invalid OTA hardware range {minimum}..={maximum}")]
    InvalidHardwareVersionRange {
        /// Declared minimum hardware version.
        minimum: u16,
        /// Declared maximum hardware version.
        maximum: u16,
    },
}
