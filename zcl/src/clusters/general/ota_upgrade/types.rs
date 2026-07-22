//! Shared OTA Upgrade command payload types.

use std::boxed::Box;
use std::vec::Vec;

use le_stream::{FromLeStream, ToLeStream};

use crate::Status;

const QUERY_JITTER_MIN: u8 = 1;
const QUERY_JITTER_MAX: u8 = 100;
const IMAGE_NOTIFY_JITTER_PAYLOAD: u8 = 0x00;
const IMAGE_NOTIFY_MANUFACTURER_PAYLOAD: u8 = 0x01;
const IMAGE_NOTIFY_IMAGE_TYPE_PAYLOAD: u8 = 0x02;
const IMAGE_NOTIFY_FILE_VERSION_PAYLOAD: u8 = 0x03;
const MAX_IMAGE_DATA_SIZE: usize = 0xFF;

/// A percentage controlling how many notified clients query the OTA server.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ToLeStream)]
#[repr(transparent)]
pub struct QueryJitter(u8);

impl QueryJitter {
    /// Smallest valid query jitter value.
    pub const MIN: u8 = QUERY_JITTER_MIN;

    /// Largest valid query jitter value.
    pub const MAX: u8 = QUERY_JITTER_MAX;

    /// Create a query jitter value when it lies in the inclusive range `1..=100`.
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> {
        if value >= Self::MIN && value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Return the raw percentage value.
    #[must_use]
    pub const fn into_inner(self) -> u8 {
        self.0
    }
}

impl FromLeStream for QueryJitter {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Self::new(u8::from_le_stream(&mut bytes)?)
    }
}

/// The manufacturer, image type, and file version identifying an OTA image.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, FromLeStream, Hash, Ord, PartialEq, PartialOrd, ToLeStream)]
pub struct ImageId {
    manufacturer_code: u16,
    image_type: u16,
    file_version: u32,
}

impl ImageId {
    /// Create an OTA image identifier.
    #[must_use]
    pub const fn new(manufacturer_code: u16, image_type: u16, file_version: u32) -> Self {
        Self {
            manufacturer_code,
            image_type,
            file_version,
        }
    }

    /// Return the Zigbee-assigned manufacturer code.
    #[must_use]
    pub const fn manufacturer_code(self) -> u16 {
        self.manufacturer_code
    }

    /// Return the manufacturer-defined image type.
    #[must_use]
    pub const fn image_type(self) -> u16 {
        self.image_type
    }

    /// Return the OTA file version.
    #[must_use]
    pub const fn file_version(self) -> u32 {
        self.file_version
    }
}

/// Payload variants of the Image Notify command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImageNotifyPayload {
    /// Notify clients without filtering by image metadata.
    QueryJitter(QueryJitter),
    /// Notify clients belonging to one manufacturer.
    Manufacturer {
        /// Percentage controlling whether a matching client queries the server.
        query_jitter: QueryJitter,
        /// Zigbee-assigned manufacturer code.
        manufacturer_code: u16,
    },
    /// Notify clients matching a manufacturer and image type.
    ImageType {
        /// Percentage controlling whether a matching client queries the server.
        query_jitter: QueryJitter,
        /// Zigbee-assigned manufacturer code.
        manufacturer_code: u16,
        /// Manufacturer-defined image type.
        image_type: u16,
    },
    /// Notify clients matching a complete image identifier.
    FileVersion {
        /// Percentage controlling whether a matching client queries the server.
        query_jitter: QueryJitter,
        /// OTA image identifier.
        image: ImageId,
    },
}

impl FromLeStream for ImageNotifyPayload {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match u8::from_le_stream(&mut bytes)? {
            IMAGE_NOTIFY_JITTER_PAYLOAD => {
                QueryJitter::from_le_stream(bytes).map(Self::QueryJitter)
            }
            IMAGE_NOTIFY_MANUFACTURER_PAYLOAD => Some(Self::Manufacturer {
                query_jitter: QueryJitter::from_le_stream(&mut bytes)?,
                manufacturer_code: u16::from_le_stream(bytes)?,
            }),
            IMAGE_NOTIFY_IMAGE_TYPE_PAYLOAD => Some(Self::ImageType {
                query_jitter: QueryJitter::from_le_stream(&mut bytes)?,
                manufacturer_code: u16::from_le_stream(&mut bytes)?,
                image_type: u16::from_le_stream(bytes)?,
            }),
            IMAGE_NOTIFY_FILE_VERSION_PAYLOAD => Some(Self::FileVersion {
                query_jitter: QueryJitter::from_le_stream(&mut bytes)?,
                image: ImageId::from_le_stream(bytes)?,
            }),
            _ => None,
        }
    }
}

impl ToLeStream for ImageNotifyPayload {
    type Iter = <Vec<u8> as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        let mut bytes = Vec::new();

        match self {
            Self::QueryJitter(query_jitter) => {
                bytes.push(IMAGE_NOTIFY_JITTER_PAYLOAD);
                bytes.extend(query_jitter.to_le_stream());
            }
            Self::Manufacturer {
                query_jitter,
                manufacturer_code,
            } => {
                bytes.push(IMAGE_NOTIFY_MANUFACTURER_PAYLOAD);
                bytes.extend(query_jitter.to_le_stream());
                bytes.extend(manufacturer_code.to_le_stream());
            }
            Self::ImageType {
                query_jitter,
                manufacturer_code,
                image_type,
            } => {
                bytes.push(IMAGE_NOTIFY_IMAGE_TYPE_PAYLOAD);
                bytes.extend(query_jitter.to_le_stream());
                bytes.extend(manufacturer_code.to_le_stream());
                bytes.extend(image_type.to_le_stream());
            }
            Self::FileVersion {
                query_jitter,
                image,
            } => {
                bytes.push(IMAGE_NOTIFY_FILE_VERSION_PAYLOAD);
                bytes.extend(query_jitter.to_le_stream());
                bytes.extend(image.to_le_stream());
            }
        }

        bytes.into_iter()
    }
}

/// Result of querying an OTA server for an image or device-specific file.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum QueryResponse {
    /// The requested image is available.
    Success {
        /// Identifier of the available image.
        image: ImageId,
        /// Total image size in bytes, including the header and all sub-elements.
        image_size: u32,
    },
    /// The server currently has no matching image.
    NoImageAvailable,
    /// The server is not authorized to upgrade the client.
    NotAuthorized,
}

impl FromLeStream for QueryResponse {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match Status::try_from(u8::from_le_stream(&mut bytes)?).ok()? {
            Status::Success => Some(Self::Success {
                image: ImageId::from_le_stream(&mut bytes)?,
                image_size: u32::from_le_stream(bytes)?,
            }),
            Status::NoImageAvailable => Some(Self::NoImageAvailable),
            Status::NotAuthorized => Some(Self::NotAuthorized),
            _ => None,
        }
    }
}

impl ToLeStream for QueryResponse {
    type Iter = <Vec<u8> as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        let mut bytes = Vec::new();

        match self {
            Self::Success { image, image_size } => {
                bytes.push(Status::Success.into());
                bytes.extend(image.to_le_stream());
                bytes.extend(image_size.to_le_stream());
            }
            Self::NoImageAvailable => bytes.push(Status::NoImageAvailable.into()),
            Self::NotAuthorized => bytes.push(Status::NotAuthorized.into()),
        }

        bytes.into_iter()
    }
}

/// A bounded block of OTA image data returned by an upgrade server.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ImageBlock {
    image: ImageId,
    file_offset: u32,
    image_data: Box<[u8]>,
}

impl ImageBlock {
    /// Largest image block representable by the one-octet data-size field.
    pub const MAX_DATA_SIZE: usize = MAX_IMAGE_DATA_SIZE;

    /// Create an image block if the image data fits in the wire data-size field.
    ///
    /// # Errors
    ///
    /// Returns the supplied image data if it exceeds [`Self::MAX_DATA_SIZE`].
    pub fn try_new(
        image: ImageId,
        file_offset: u32,
        image_data: Box<[u8]>,
    ) -> Result<Self, Box<[u8]>> {
        if image_data.len() <= Self::MAX_DATA_SIZE {
            Ok(Self {
                image,
                file_offset,
                image_data,
            })
        } else {
            Err(image_data)
        }
    }

    /// Return the OTA image identifier.
    #[must_use]
    pub const fn image(&self) -> ImageId {
        self.image
    }

    /// Return the byte offset of this block within the OTA file.
    #[must_use]
    pub const fn file_offset(&self) -> u32 {
        self.file_offset
    }

    /// Return the OTA image data in this block.
    #[must_use]
    pub fn image_data(&self) -> &[u8] {
        &self.image_data
    }
}

impl FromLeStream for ImageBlock {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let image = ImageId::from_le_stream(&mut bytes)?;
        let file_offset = u32::from_le_stream(&mut bytes)?;
        let data_size = usize::from(u8::from_le_stream(&mut bytes)?);
        let image_data: Box<[u8]> = bytes.by_ref().take(data_size).collect();

        (image_data.len() == data_size).then_some(Self {
            image,
            file_offset,
            image_data,
        })
    }
}

impl ToLeStream for ImageBlock {
    type Iter = <Vec<u8> as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        let data_size = u8::try_from(self.image_data.len())
            .expect("ImageBlock constructor limits image data to 255 bytes");
        let mut bytes = Vec::new();
        bytes.extend(self.image.to_le_stream());
        bytes.extend(self.file_offset.to_le_stream());
        bytes.push(data_size);
        bytes.extend(self.image_data);
        bytes.into_iter()
    }
}

/// Timing information returned when an OTA server temporarily has no block data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, FromLeStream, Hash, Ord, PartialEq, PartialOrd, ToLeStream)]
pub struct WaitForData {
    current_time: u32,
    request_time: u32,
    minimum_block_period: u16,
}

impl WaitForData {
    /// Create wait timing information.
    #[must_use]
    pub const fn new(current_time: u32, request_time: u32, minimum_block_period: u16) -> Self {
        Self {
            current_time,
            request_time,
            minimum_block_period,
        }
    }

    /// Return the server's current time.
    #[must_use]
    pub const fn current_time(self) -> u32 {
        self.current_time
    }

    /// Return the earliest time at which the client should request more data.
    #[must_use]
    pub const fn request_time(self) -> u32 {
        self.request_time
    }

    /// Return the new minimum delay between block requests in seconds.
    #[must_use]
    pub const fn minimum_block_period(self) -> u16 {
        self.minimum_block_period
    }
}

/// Payload variants of the Image Block Response command.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImageBlockResponsePayload {
    /// A block of OTA image data is available.
    Success(ImageBlock),
    /// The client should wait before requesting data again.
    WaitForData(WaitForData),
    /// The server has aborted the image transfer.
    Abort,
}

impl FromLeStream for ImageBlockResponsePayload {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match Status::try_from(u8::from_le_stream(&mut bytes)?).ok()? {
            Status::Success => ImageBlock::from_le_stream(bytes).map(Self::Success),
            Status::WaitForData => WaitForData::from_le_stream(bytes).map(Self::WaitForData),
            Status::Abort => Some(Self::Abort),
            _ => None,
        }
    }
}

impl ToLeStream for ImageBlockResponsePayload {
    type Iter = <Vec<u8> as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        let mut bytes = Vec::new();

        match self {
            Self::Success(block) => {
                bytes.push(Status::Success.into());
                bytes.extend(block.to_le_stream());
            }
            Self::WaitForData(wait) => {
                bytes.push(Status::WaitForData.into());
                bytes.extend(wait.to_le_stream());
            }
            Self::Abort => bytes.push(Status::Abort.into()),
        }

        bytes.into_iter()
    }
}

/// Status sent by an OTA client when it finishes or terminates a download.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum UpgradeEndStatus {
    /// The image was downloaded and validated successfully.
    Success = 0x00,
    /// The client aborted the upgrade process.
    Abort = 0x95,
    /// The downloaded image failed validation.
    InvalidImage = 0x96,
    /// The client requires another image before it can upgrade.
    RequireMoreImage = 0x99,
}

impl FromLeStream for UpgradeEndStatus {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Self::try_from(u8::from_le_stream(&mut bytes)?).ok()
    }
}

impl ToLeStream for UpgradeEndStatus {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};

    use super::{
        ImageBlock, ImageBlockResponsePayload, ImageId, ImageNotifyPayload, QueryJitter,
        QueryResponse, WaitForData,
    };

    const IMAGE: ImageId = ImageId::new(0x1234, 0x5678, 0x90AB_CDEF);

    fn round_trip<T>(value: T)
    where
        T: Clone + core::fmt::Debug + Eq + FromLeStream + ToLeStream,
    {
        let bytes = value.clone().to_le_stream();
        assert_eq!(T::from_le_stream(bytes), Some(value));
    }

    #[test]
    fn validates_query_jitter_range() {
        assert!(QueryJitter::new(QueryJitter::MIN).is_some());
        assert!(QueryJitter::new(QueryJitter::MAX).is_some());
        assert!(QueryJitter::new(QueryJitter::MIN - 1).is_none());
        assert!(QueryJitter::new(QueryJitter::MAX + 1).is_none());
    }

    #[test]
    fn image_notify_payloads_round_trip() {
        let query_jitter = QueryJitter::new(50).expect("valid query jitter");
        round_trip(ImageNotifyPayload::QueryJitter(query_jitter));
        round_trip(ImageNotifyPayload::Manufacturer {
            query_jitter,
            manufacturer_code: IMAGE.manufacturer_code(),
        });
        round_trip(ImageNotifyPayload::ImageType {
            query_jitter,
            manufacturer_code: IMAGE.manufacturer_code(),
            image_type: IMAGE.image_type(),
        });
        round_trip(ImageNotifyPayload::FileVersion {
            query_jitter,
            image: IMAGE,
        });
    }

    #[test]
    fn query_responses_round_trip() {
        round_trip(QueryResponse::Success {
            image: IMAGE,
            image_size: 4096,
        });
        round_trip(QueryResponse::NoImageAvailable);
        round_trip(QueryResponse::NotAuthorized);
    }

    #[test]
    fn block_response_payloads_round_trip() {
        let block =
            ImageBlock::try_new(IMAGE, 128, Box::from([1, 2, 3, 4])).expect("small image block");
        round_trip(ImageBlockResponsePayload::Success(block));
        round_trip(ImageBlockResponsePayload::WaitForData(WaitForData::new(
            100, 110, 2,
        )));
        round_trip(ImageBlockResponsePayload::Abort);
    }

    #[test]
    fn rejects_oversized_image_block() {
        let image_data = vec![0; ImageBlock::MAX_DATA_SIZE + 1].into_boxed_slice();
        assert!(ImageBlock::try_new(IMAGE, 0, image_data).is_err());
    }
}
