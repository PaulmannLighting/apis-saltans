//! Validated Zigbee OTA upgrade image files.

use bytes::Bytes;
use le_stream::FromLeStream;
use thiserror::Error;
use zb_core::IeeeAddress;
use zb_zcl::ota_upgrade::ImageId;

const OTA_FILE_IDENTIFIER: u32 = 0x0bee_f11e;
const SUPPORTED_HEADER_VERSION: u16 = 0x0100;
const BASE_HEADER_LENGTH: usize = 56;
const HEADER_STRING_LENGTH: usize = 32;
const SECURITY_CREDENTIAL_VERSION_PRESENT: u16 = 0x0001;
const UPGRADE_FILE_DESTINATION_PRESENT: u16 = 0x0002;
const HARDWARE_VERSIONS_PRESENT: u16 = 0x0004;
const SUPPORTED_FIELD_CONTROL: u16 = SECURITY_CREDENTIAL_VERSION_PRESENT
    | UPGRADE_FILE_DESTINATION_PRESENT
    | HARDWARE_VERSIONS_PRESENT;
const SECURITY_CREDENTIAL_VERSION_LENGTH: usize = 1;
const UPGRADE_FILE_DESTINATION_LENGTH: usize = 8;
const HARDWARE_VERSIONS_LENGTH: usize = 4;

/// A validated, complete Zigbee OTA upgrade image file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image {
    bytes: Bytes,
    header_length: u16,
    id: ImageId,
    zigbee_stack_version: u16,
    header_string: [u8; HEADER_STRING_LENGTH],
    security_credential_version: Option<u8>,
    upgrade_file_destination: Option<IeeeAddress>,
    hardware_versions: Option<(u16, u16)>,
}

impl Image {
    /// Parse and validate a complete Zigbee OTA upgrade image file.
    ///
    /// # Errors
    ///
    /// Returns [`ParseImageError`] if the file header is malformed, unsupported, or inconsistent
    /// with the supplied bytes.
    pub fn parse(bytes: Bytes) -> Result<Self, ParseImageError> {
        if bytes.len() < BASE_HEADER_LENGTH {
            return Err(ParseImageError::TruncatedHeader);
        }

        let mut cursor = bytes.iter().copied();
        let file_identifier = read::<u32, _>(&mut cursor)?;
        if file_identifier != OTA_FILE_IDENTIFIER {
            return Err(ParseImageError::InvalidFileIdentifier(file_identifier));
        }

        let header_version = read::<u16, _>(&mut cursor)?;
        if header_version != SUPPORTED_HEADER_VERSION {
            return Err(ParseImageError::UnsupportedHeaderVersion(header_version));
        }

        let header_length = read::<u16, _>(&mut cursor)?;
        let field_control = read::<u16, _>(&mut cursor)?;
        let unsupported_fields = field_control & !SUPPORTED_FIELD_CONTROL;
        if unsupported_fields != 0 {
            return Err(ParseImageError::UnsupportedFieldControl(unsupported_fields));
        }

        let id = ImageId::new(
            read::<u16, _>(&mut cursor)?,
            read::<u16, _>(&mut cursor)?,
            read::<u32, _>(&mut cursor)?,
        );
        let zigbee_stack_version = read::<u16, _>(&mut cursor)?;
        let mut header_string = [0; HEADER_STRING_LENGTH];
        for octet in &mut header_string {
            *octet = read::<u8, _>(&mut cursor)?;
        }
        let total_image_size = read::<u32, _>(&mut cursor)?;

        let expected_header_length = BASE_HEADER_LENGTH
            + optional_length(
                field_control,
                SECURITY_CREDENTIAL_VERSION_PRESENT,
                SECURITY_CREDENTIAL_VERSION_LENGTH,
            )
            + optional_length(
                field_control,
                UPGRADE_FILE_DESTINATION_PRESENT,
                UPGRADE_FILE_DESTINATION_LENGTH,
            )
            + optional_length(
                field_control,
                HARDWARE_VERSIONS_PRESENT,
                HARDWARE_VERSIONS_LENGTH,
            );
        if usize::from(header_length) != expected_header_length {
            return Err(ParseImageError::InvalidHeaderLength {
                declared: header_length,
                expected: expected_header_length,
            });
        }

        let actual_image_size =
            u32::try_from(bytes.len()).map_err(|_| ParseImageError::ImageTooLarge)?;
        if total_image_size != actual_image_size {
            return Err(ParseImageError::InvalidImageSize {
                declared: total_image_size,
                actual: actual_image_size,
            });
        }

        let security_credential_version = field_control
            .then_read(SECURITY_CREDENTIAL_VERSION_PRESENT, || {
                read::<u8, _>(&mut cursor)
            })?;
        let upgrade_file_destination = field_control
            .then_read(UPGRADE_FILE_DESTINATION_PRESENT, || {
                read::<IeeeAddress, _>(&mut cursor)
            })?;
        let hardware_versions = field_control.then_read(HARDWARE_VERSIONS_PRESENT, || {
            Ok((read::<u16, _>(&mut cursor)?, read::<u16, _>(&mut cursor)?))
        })?;

        if let Some((minimum, maximum)) = hardware_versions
            && minimum > maximum
        {
            return Err(ParseImageError::InvalidHardwareVersionRange { minimum, maximum });
        }

        Ok(Self {
            bytes,
            header_length,
            id,
            zigbee_stack_version,
            header_string,
            security_credential_version,
            upgrade_file_destination,
            hardware_versions,
        })
    }

    /// Return the manufacturer, image type, and file version from the image header.
    #[must_use]
    pub const fn id(&self) -> ImageId {
        self.id
    }

    /// Return the complete image size, including the OTA header.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Return whether the image contains no bytes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Return the OTA header length in bytes.
    #[must_use]
    pub const fn header_length(&self) -> u16 {
        self.header_length
    }

    /// Return the Zigbee stack version from the OTA header.
    #[must_use]
    pub const fn zigbee_stack_version(&self) -> u16 {
        self.zigbee_stack_version
    }

    /// Return the raw 32-byte header string.
    #[must_use]
    pub const fn header_string(&self) -> &[u8; HEADER_STRING_LENGTH] {
        &self.header_string
    }

    /// Return the optional security credential version.
    #[must_use]
    pub const fn security_credential_version(&self) -> Option<u8> {
        self.security_credential_version
    }

    /// Return the optional IEEE address to which this file is restricted.
    #[must_use]
    pub const fn upgrade_file_destination(&self) -> Option<IeeeAddress> {
        self.upgrade_file_destination
    }

    /// Return the optional inclusive hardware-version range.
    #[must_use]
    pub const fn hardware_versions(&self) -> Option<(u16, u16)> {
        self.hardware_versions
    }

    /// Return the complete serialized OTA image.
    #[must_use]
    pub const fn as_bytes(&self) -> &Bytes {
        &self.bytes
    }

    pub(super) fn image_size(&self) -> u32 {
        u32::try_from(self.bytes.len()).expect("validated OTA image length fits in u32")
    }

    pub(super) fn supports_hardware(&self, hardware_version: Option<u16>) -> bool {
        match (self.hardware_versions, hardware_version) {
            (Some((minimum, maximum)), Some(version)) => (minimum..=maximum).contains(&version),
            (Some(_), None) => false,
            (None, _) => true,
        }
    }
}

impl TryFrom<Bytes> for Image {
    type Error = ParseImageError;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::parse(bytes)
    }
}

/// Error returned while parsing a Zigbee OTA upgrade image.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ParseImageError {
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
    /// The declared header length does not match the optional fields.
    #[error("invalid OTA header length {declared}; expected {expected}")]
    InvalidHeaderLength {
        /// Length declared in the OTA header.
        declared: u16,
        /// Length implied by the field-control bits.
        expected: usize,
    },
    /// The byte buffer is too large for the OTA file-size field.
    #[error("OTA image is larger than the 32-bit file-size field")]
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

trait FieldControlExt {
    fn then_read<T, F>(self, bit: u16, read: F) -> Result<Option<T>, ParseImageError>
    where
        F: FnOnce() -> Result<T, ParseImageError>;
}

impl FieldControlExt for u16 {
    fn then_read<T, F>(self, bit: u16, read: F) -> Result<Option<T>, ParseImageError>
    where
        F: FnOnce() -> Result<T, ParseImageError>,
    {
        if self & bit == 0 {
            Ok(None)
        } else {
            read().map(Some)
        }
    }
}

fn read<T, I>(bytes: &mut I) -> Result<T, ParseImageError>
where
    T: FromLeStream,
    I: Iterator<Item = u8>,
{
    T::from_le_stream(bytes).ok_or(ParseImageError::TruncatedHeader)
}

const fn optional_length(field_control: u16, bit: u16, length: usize) -> usize {
    if field_control & bit == 0 { 0 } else { length }
}

#[cfg(test)]
mod tests {
    use bytes::{BufMut, BytesMut};

    use super::{
        BASE_HEADER_LENGTH, HARDWARE_VERSIONS_PRESENT, Image, OTA_FILE_IDENTIFIER, ParseImageError,
        SUPPORTED_HEADER_VERSION,
    };

    const MANUFACTURER_CODE: u16 = 0x1234;
    const IMAGE_TYPE: u16 = 0x5678;
    const FILE_VERSION: u32 = 0x0102_0304;
    const STACK_VERSION: u16 = 0x0002;
    const PAYLOAD: &[u8] = &[1, 2, 3, 4];

    #[test]
    fn parses_complete_image() {
        let image = Image::parse(image_bytes(0, None)).expect("valid OTA image");

        assert_eq!(image.id().manufacturer_code(), MANUFACTURER_CODE);
        assert_eq!(image.id().image_type(), IMAGE_TYPE);
        assert_eq!(image.id().file_version(), FILE_VERSION);
        assert_eq!(image.zigbee_stack_version(), STACK_VERSION);
        assert_eq!(
            image.header_length(),
            u16::try_from(BASE_HEADER_LENGTH).expect("fixed header length fits u16")
        );
        assert_eq!(image.as_bytes().len(), BASE_HEADER_LENGTH + PAYLOAD.len());
    }

    #[test]
    fn validates_hardware_range() {
        let image = Image::parse(image_bytes(HARDWARE_VERSIONS_PRESENT, Some((10, 20))))
            .expect("valid OTA image");

        assert!(image.supports_hardware(Some(10)));
        assert!(image.supports_hardware(Some(20)));
        assert!(!image.supports_hardware(Some(9)));
        assert!(!image.supports_hardware(None));
    }

    #[test]
    fn rejects_inconsistent_total_size() {
        let mut bytes = image_bytes(0, None).to_vec();
        bytes.push(0);

        assert!(matches!(
            Image::parse(bytes.into()),
            Err(ParseImageError::InvalidImageSize { .. })
        ));
    }

    fn image_bytes(field_control: u16, hardware: Option<(u16, u16)>) -> bytes::Bytes {
        let header_length =
            BASE_HEADER_LENGTH + hardware.map_or(0, |_| super::HARDWARE_VERSIONS_LENGTH);
        let total_length = header_length + PAYLOAD.len();
        let mut bytes = BytesMut::with_capacity(total_length);
        bytes.put_u32_le(OTA_FILE_IDENTIFIER);
        bytes.put_u16_le(SUPPORTED_HEADER_VERSION);
        bytes.put_u16_le(u16::try_from(header_length).expect("test header length fits u16"));
        bytes.put_u16_le(field_control);
        bytes.put_u16_le(MANUFACTURER_CODE);
        bytes.put_u16_le(IMAGE_TYPE);
        bytes.put_u32_le(FILE_VERSION);
        bytes.put_u16_le(STACK_VERSION);
        bytes.extend_from_slice(&[0; super::HEADER_STRING_LENGTH]);
        bytes.put_u32_le(u32::try_from(total_length).expect("test image length fits u32"));
        if let Some((minimum, maximum)) = hardware {
            bytes.put_u16_le(minimum);
            bytes.put_u16_le(maximum);
        }
        bytes.extend_from_slice(PAYLOAD);
        bytes.freeze()
    }
}
