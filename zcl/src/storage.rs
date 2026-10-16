//! Storage selected by the optional `alloc` feature.
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, vec::Vec};

#[cfg(not(feature = "alloc"))]
use const_env::env_item;

#[cfg(not(feature = "alloc"))]
pub use self::bounded::{RawList, Stream};
#[cfg(not(feature = "alloc"))]
pub use self::wire::WireBytes;

#[cfg(not(feature = "alloc"))]
mod bounded;
#[cfg(test)]
mod tests;
#[cfg(not(feature = "alloc"))]
mod wire;

/// Maximum records storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_RECORDS_CAPACITY")]
pub const RECORDS_CAPACITY: usize = 4;

/// Maximum attribute ids storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_ATTRIBUTE_IDS_CAPACITY")]
pub const ATTRIBUTE_IDS_CAPACITY: usize = 64;

/// Maximum command ids storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_COMMAND_IDS_CAPACITY")]
pub const COMMAND_IDS_CAPACITY: usize = 64;

/// Maximum indexes storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_INDEXES_CAPACITY")]
pub const INDEXES_CAPACITY: usize = 15;

/// Maximum image data storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_IMAGE_DATA_CAPACITY")]
pub const IMAGE_DATA_CAPACITY: usize = 255;

/// Maximum extensions storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_EXTENSIONS_CAPACITY")]
pub const EXTENSIONS_CAPACITY: usize = 16;

/// Maximum reportable change storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_REPORTABLE_CHANGE_CAPACITY")]
pub const REPORTABLE_CHANGE_CAPACITY: usize = 8;

/// Maximum payload storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_PAYLOAD_CAPACITY")]
pub const PAYLOAD_CAPACITY: usize = 1024;

/// Maximum wire storage without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZCL_WIRE_CAPACITY")]
pub const WIRE_CAPACITY: usize = 269;

#[cfg(not(feature = "alloc"))]
const _: () = {
    assert!(
        RECORDS_CAPACITY >= 1,
        "ZCL RECORDS capacity must be at least 1"
    );
    assert!(
        ATTRIBUTE_IDS_CAPACITY >= 1,
        "ZCL ATTRIBUTE_IDS capacity must be at least 1"
    );
    assert!(
        COMMAND_IDS_CAPACITY >= 1,
        "ZCL COMMAND_IDS capacity must be at least 1"
    );
    assert!(
        INDEXES_CAPACITY >= 1,
        "ZCL INDEXES capacity must be at least 1"
    );
    assert!(
        IMAGE_DATA_CAPACITY >= 1,
        "ZCL IMAGE_DATA capacity must be at least 1"
    );
    assert!(
        EXTENSIONS_CAPACITY >= 1,
        "ZCL EXTENSIONS capacity must be at least 1"
    );
    assert!(
        REPORTABLE_CHANGE_CAPACITY >= 1,
        "ZCL REPORTABLE_CHANGE capacity must be at least 1"
    );
    assert!(
        PAYLOAD_CAPACITY >= 1,
        "ZCL PAYLOAD capacity must be at least 1"
    );
    assert!(
        WIRE_CAPACITY >= 269,
        "ZCL WIRE capacity must be at least 269"
    );
};

/// Storage for `Records` with allocation.
#[cfg(feature = "alloc")]
pub type Records<T> = Box<[T]>;
/// Bounded storage for `Records`.
#[cfg(not(feature = "alloc"))]
pub type Records<T> = RawList<T, RECORDS_CAPACITY>;

/// Storage for `AttributeIds` with allocation.
#[cfg(feature = "alloc")]
pub type AttributeIds = Box<[u16]>;
/// Bounded storage for `AttributeIds`.
#[cfg(not(feature = "alloc"))]
pub type AttributeIds = RawList<u16, ATTRIBUTE_IDS_CAPACITY>;

/// Storage for `CommandIds` with allocation.
#[cfg(feature = "alloc")]
pub type CommandIds = Box<[u8]>;
/// Bounded storage for `CommandIds`.
#[cfg(not(feature = "alloc"))]
pub type CommandIds = RawList<u8, COMMAND_IDS_CAPACITY>;

/// Storage for `Indexes` with allocation.
#[cfg(feature = "alloc")]
pub type Indexes = Box<[u16]>;
/// Bounded storage for `Indexes`.
#[cfg(not(feature = "alloc"))]
pub type Indexes = RawList<u16, INDEXES_CAPACITY>;

/// Storage for `ImageData` with allocation.
#[cfg(feature = "alloc")]
pub type ImageData = Box<[u8]>;
/// Bounded storage for `ImageData`.
#[cfg(not(feature = "alloc"))]
pub type ImageData = RawList<u8, IMAGE_DATA_CAPACITY>;

/// Storage for `Extensions` with allocation.
#[cfg(feature = "alloc")]
pub type Extensions = Vec<crate::scenes::SceneTableExtension>;
/// Bounded storage for `Extensions`.
#[cfg(not(feature = "alloc"))]
pub type Extensions = RawList<crate::scenes::SceneTableExtension, EXTENSIONS_CAPACITY>;

/// Storage for `ReportableChange` with allocation.
#[cfg(feature = "alloc")]
pub type ReportableChange = bytes::Bytes;
/// Bounded storage for `ReportableChange`.
#[cfg(not(feature = "alloc"))]
pub type ReportableChange = RawList<u8, REPORTABLE_CHANGE_CAPACITY>;

/// Storage for `Payload` with allocation.
#[cfg(feature = "alloc")]
pub type Payload = bytes::Bytes;
/// Bounded storage for `Payload`.
#[cfg(not(feature = "alloc"))]
pub type Payload = RawList<u8, PAYLOAD_CAPACITY>;

/// Storage for `Owned` with allocation.
#[cfg(feature = "alloc")]
pub type Owned<T> = Box<T>;
/// Bounded storage for `Owned`.
#[cfg(not(feature = "alloc"))]
pub type Owned<T> = T;

/// Temporary serialization bytes for fixed-size protocol payloads.
#[cfg(feature = "alloc")]
pub type WireBytes = Vec<u8>;

/// A configured ZCL storage capacity was exceeded.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("ZCL collection capacity exceeded: {capacity} elements")]
pub struct CapacityError {
    /// Maximum element count.
    pub capacity: usize,
}

#[cfg_attr(
    not(feature = "alloc"),
    expect(clippy::missing_const_for_fn, reason = "allocating mode allocates")
)]
pub(crate) fn own<T>(value: T) -> Owned<T> {
    #[cfg(feature = "alloc")]
    {
        Box::new(value)
    }
    #[cfg(not(feature = "alloc"))]
    {
        value
    }
}
#[cfg_attr(
    feature = "alloc",
    expect(clippy::boxed_local, reason = "consume the existing boxed API")
)]
#[cfg_attr(
    not(feature = "alloc"),
    expect(clippy::missing_const_for_fn, reason = "allocating mode deallocates")
)]
pub(crate) fn unpack<T>(value: Owned<T>) -> T {
    #[cfg(feature = "alloc")]
    {
        *value
    }
    #[cfg(not(feature = "alloc"))]
    {
        value
    }
}

#[cfg(test)]
pub(crate) fn test_records<T, I>(items: I) -> Records<T>
where
    I: IntoIterator<Item = T>,
{
    #[cfg(feature = "alloc")]
    {
        items.into_iter().collect()
    }
    #[cfg(not(feature = "alloc"))]
    {
        Records::<T>::try_from_iter(items).expect("test collection fits")
    }
}

#[cfg(test)]
pub(crate) fn test_image_data<I>(items: I) -> ImageData
where
    I: IntoIterator<Item = u8>,
{
    #[cfg(feature = "alloc")]
    {
        items.into_iter().collect()
    }
    #[cfg(not(feature = "alloc"))]
    {
        ImageData::try_from_iter(items).expect("test collection fits")
    }
}
