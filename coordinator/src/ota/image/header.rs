use bytes::{Bytes, BytesMut};
use heapless::String;
use le_stream::FromLeStream;
use zb_core::IeeeAddress;
use zb_zcl::ota_upgrade::ImageId;

use super::{FieldControl, ParseImageError};

const OTA_FILE_IDENTIFIER: u32 = 0x0bee_f11e;
const SUPPORTED_HEADER_VERSION: u16 = 0x0100;
const BASE_HEADER_LENGTH: usize = 56;
const HEADER_STRING_LENGTH: usize = 32;

/// Serialized bytes in the mandatory portion of an OTA image header.
pub type BaseHeaderBytes = [u8; BASE_HEADER_LENGTH];

/// Human-readable ASCII portion of a fixed-width OTA header string.
pub type HeaderString = String<HEADER_STRING_LENGTH>;

/// Parsed metadata and serialized bytes of a Zigbee OTA image header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Header {
    bytes: Bytes,
    length: u16,
    field_control: FieldControl,
    id: ImageId,
    zigbee_stack_version: u16,
    description: HeaderString,
    security_credential_version: Option<u8>,
    upgrade_file_destination: Option<IeeeAddress>,
    hardware_versions: Option<(u16, u16)>,
    total_image_size: u32,
    image_length: usize,
}

pub(super) struct HeaderBuilder {
    bytes: BaseHeaderBytes,
    length: u16,
    field_control: FieldControl,
    id: ImageId,
    zigbee_stack_version: u16,
    description: HeaderString,
    total_image_size: u32,
}

impl Header {
    /// Return the serialized OTA header bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the OTA header length in bytes.
    #[must_use]
    pub const fn header_length(&self) -> u16 {
        self.length
    }

    /// Return the optional fields present in this header.
    #[must_use]
    pub const fn field_control(&self) -> FieldControl {
        self.field_control
    }

    /// Return the manufacturer, image type, and file version.
    #[must_use]
    pub const fn id(&self) -> ImageId {
        self.id
    }

    /// Return the Zigbee stack version.
    #[must_use]
    pub const fn zigbee_stack_version(&self) -> u16 {
        self.zigbee_stack_version
    }

    /// Return the human-readable string before its null terminator.
    #[must_use]
    pub const fn header_string(&self) -> &HeaderString {
        &self.description
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

    /// Return the complete OTA image size, including this header.
    #[must_use]
    pub const fn total_image_size(&self) -> u32 {
        self.total_image_size
    }

    pub(super) const fn image_length(&self) -> usize {
        self.image_length
    }

    pub(super) fn supports_hardware(&self, hardware_version: Option<u16>) -> bool {
        match (self.hardware_versions(), hardware_version) {
            (Some((minimum, maximum)), Some(version)) => (minimum..=maximum).contains(&version),
            (Some(_), None) => false,
            (None, _) => true,
        }
    }
}

impl HeaderBuilder {
    pub(super) fn parse(bytes: BaseHeaderBytes) -> Result<Self, ParseImageError> {
        let mut cursor = bytes.iter().copied();
        let file_identifier = read::<u32, _>(&mut cursor)?;
        if file_identifier != OTA_FILE_IDENTIFIER {
            return Err(ParseImageError::InvalidFileIdentifier(file_identifier));
        }

        let header_version = read::<u16, _>(&mut cursor)?;
        if header_version != SUPPORTED_HEADER_VERSION {
            return Err(ParseImageError::UnsupportedHeaderVersion(header_version));
        }

        let length = read::<u16, _>(&mut cursor)?;
        let field_control_bits = read::<u16, _>(&mut cursor)?;
        let field_control = FieldControl::from_bits(field_control_bits).ok_or_else(|| {
            ParseImageError::UnsupportedFieldControl(
                field_control_bits & !FieldControl::all().bits(),
            )
        })?;
        let expected_length = BASE_HEADER_LENGTH + field_control.optional_header_length();
        if usize::from(length) != expected_length {
            return Err(ParseImageError::InvalidHeaderLength {
                declared: length,
                expected: expected_length,
            });
        }

        let id = ImageId::new(
            read::<u16, _>(&mut cursor)?,
            read::<u16, _>(&mut cursor)?,
            read::<u32, _>(&mut cursor)?,
        );
        let zigbee_stack_version = read::<u16, _>(&mut cursor)?;
        let description = parse_header_string(&mut cursor)?;
        let total_image_size = read::<u32, _>(&mut cursor)?;

        Ok(Self {
            bytes,
            length,
            field_control,
            id,
            zigbee_stack_version,
            description,
            total_image_size,
        })
    }

    pub(super) const fn optional_header_length(&self) -> usize {
        self.field_control.optional_header_length()
    }

    pub(super) fn finish(
        self,
        optional_bytes: &[u8],
        image_length: usize,
    ) -> Result<Header, ParseImageError> {
        let actual_image_size =
            u32::try_from(image_length).map_err(|_| ParseImageError::ImageTooLarge)?;
        if self.total_image_size != actual_image_size {
            return Err(ParseImageError::InvalidImageSize {
                declared: self.total_image_size,
                actual: actual_image_size,
            });
        }

        let mut cursor = optional_bytes.iter().copied();
        let security_credential_version = self
            .field_control
            .contains(FieldControl::SECURITY_CREDENTIAL_VERSION)
            .then(|| read::<u8, _>(&mut cursor))
            .transpose()?;
        let upgrade_file_destination = self
            .field_control
            .contains(FieldControl::UPGRADE_FILE_DESTINATION)
            .then(|| read::<IeeeAddress, _>(&mut cursor))
            .transpose()?;
        let hardware_versions = self
            .field_control
            .contains(FieldControl::HARDWARE_VERSIONS)
            .then(|| {
                Ok::<_, ParseImageError>((
                    read::<u16, _>(&mut cursor)?,
                    read::<u16, _>(&mut cursor)?,
                ))
            })
            .transpose()?;
        if let Some((minimum, maximum)) = hardware_versions
            && minimum > maximum
        {
            return Err(ParseImageError::InvalidHardwareVersionRange { minimum, maximum });
        }

        let mut bytes = BytesMut::with_capacity(usize::from(self.length));
        bytes.extend_from_slice(&self.bytes);
        bytes.extend_from_slice(optional_bytes);

        Ok(Header {
            bytes: bytes.freeze(),
            length: self.length,
            field_control: self.field_control,
            id: self.id,
            zigbee_stack_version: self.zigbee_stack_version,
            description: self.description,
            security_credential_version,
            upgrade_file_destination,
            hardware_versions,
            total_image_size: self.total_image_size,
            image_length,
        })
    }
}

fn read<T, I>(bytes: &mut I) -> Result<T, ParseImageError>
where
    T: FromLeStream,
    I: Iterator<Item = u8>,
{
    T::from_le_stream(bytes).ok_or(ParseImageError::TruncatedHeader)
}

fn parse_header_string<I>(bytes: &mut I) -> Result<HeaderString, ParseImageError>
where
    I: Iterator<Item = u8>,
{
    let mut header_string = HeaderString::new();
    let mut terminated = false;
    for _ in 0..HEADER_STRING_LENGTH {
        let octet = read::<u8, _>(bytes)?;
        if terminated {
            continue;
        }
        if octet == 0 {
            terminated = true;
        } else if octet.is_ascii() {
            header_string
                .push(char::from(octet))
                .map_err(|_| ParseImageError::InvalidHeaderString)?;
        } else {
            return Err(ParseImageError::InvalidHeaderString);
        }
    }
    if terminated {
        Ok(header_string)
    } else {
        Err(ParseImageError::InvalidHeaderString)
    }
}
