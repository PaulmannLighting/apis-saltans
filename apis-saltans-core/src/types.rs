//! Common types used across the protocol.

use core::convert::Infallible;

use le_stream::{FromLeStream, ToLeStream};
use repr_discriminant::ReprDiscriminant;

pub use self::analog::{
    Int8, Int16, Int24, Int32, Int40, Int48, Int56, Int64, Uint8, Uint16, Uint24, Uint32, Uint40,
    Uint48, Uint56, Uint64,
};
pub use self::channel_list::{ChannelList, Pages};
pub use self::channels_field::ChannelsField;
pub use self::composite::{OctStr, String};
pub use self::configuration_bitmask::ConfigurationBitmask;
pub use self::discrete::{
    Bool, Data8, Data16, Data24, Data32, Data40, Data48, Data56, Data64, Date, TimeOfDay,
    TryFromNaiveDateError, TryFromNaiveTimeError, TryIntoNaiveDateError, TryIntoNaiveTimeError,
    UtcTime,
};
pub use self::null::{NoData, Unknown};
use crate::IeeeAddress;

mod analog;
mod channel_list;
mod channels_field;
mod composite;
mod configuration_bitmask;
mod discrete;
mod null;
pub mod tlv;

/// Commonly used type identifiers.
#[cfg_attr(
    feature = "serde",
    expect(clippy::unsafe_derive_deserialize),
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ReprDiscriminant, FromLeStream, ToLeStream,
)]
#[repr(u8)]
pub enum Type {
    /// Unknown type.
    Unknown = 0xff,

    /// No data type.
    NoData = 0x00,

    /// 8-bit data.
    Data8([u8; 1]) = 0x08,

    /// 16-bit data.
    Data16([u8; 2]) = 0x09,

    /// 24-bit data.
    Data24([u8; 3]) = 0x0a,

    /// 32-bit data.
    Data32([u8; 4]) = 0x0b,

    /// 40-bit data.
    Data40([u8; 5]) = 0x0c,

    /// 48-bit data.
    Data48([u8; 6]) = 0x0d,

    /// 56-bit data.
    Data56([u8; 7]) = 0x0e,

    /// 64-bit data.
    Data64([u8; 8]) = 0x0f,

    /// Boolean type.
    Boolean(Bool) = 0x10,

    /// 8-bit map.
    Map8(u8) = 0x18,

    /// 16-bit map.
    Map16(u16) = 0x19,

    /// 32-bit map.
    Map32(u32) = 0x1b,

    /// 64-bit map.
    Map64(u64) = 0x1c,

    /// 8-bit unsigned integer.
    Uint8(Uint8) = 0x20,

    /// 16-bit unsigned integer.
    Uint16(Uint16) = 0x21,

    /// 24-bit unsigned integer.
    Uint24(Uint24) = 0x22,

    /// 32-bit unsigned integer.
    Uint32(Uint32) = 0x23,

    /// 40-bit unsigned integer.
    Uint40(Uint40) = 0x24,

    /// 48-bit unsigned integer.
    Uint48(Uint48) = 0x25,

    /// 56-bit unsigned integer.
    Uint56(Uint56) = 0x26,

    /// 64-bit unsigned integer.
    Uint64(Uint64) = 0x27,

    /// 8-bit signed integer.
    Int8(Int8) = 0x28,

    /// 16-bit signed integer.
    Int16(Int16) = 0x29,

    /// 24-bit signed integer.
    Int24(Int24) = 0x2a,

    /// 32-bit signed integer.
    Int32(Int32) = 0x2b,

    /// 40-bit signed integer.
    Int40(Int40) = 0x2c,

    /// 48-bit signed integer.
    Int48(Int48) = 0x2d,

    /// 56-bit signed integer.
    Int56(Int56) = 0x2e,

    /// 64-bit signed integer.
    Int64(Int64) = 0x2f,

    ///8-bit enumerated type.
    Enum8(Uint8) = 0x30,

    /// 16-bit enumerated type.
    Enum16(Uint16) = 0x31,

    /// Octet string.
    OctetString(OctStr) = 0x41,

    /// String type.
    String(String) = 0x42,

    /// Time of day.
    TimeOfDay(TimeOfDay) = 0xe0,

    /// Date type.
    Date(Date) = 0xe1,

    /// UTC time.
    UtcTime(UtcTime) = 0xe2,

    /// Cluster ID.
    ClusterId(u16) = 0xe8,

    /// Attribute ID.
    AttributeId(u16) = 0xe9,

    /// `BACnet` Object ID.
    BacnetObjectId(u32) = 0xea,

    /// IEEE Address.
    IeeeAddress(IeeeAddress) = 0xf0,

    /// 128-bit Key.
    Key128([u8; 16]) = 0xf1,
}

/// Conversion implementation for types that require `From<Type, Error: Into<Type>>`.
impl From<Infallible> for Type {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}
