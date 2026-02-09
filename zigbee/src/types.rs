//! Common types used across the protocol.

use std::vec::IntoIter;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};
use macaddr::MacAddr8;
use repr_discriminant::ReprDiscriminant;

pub use self::analog::{
    Int8, Int16, Int24, Int32, Int40, Int48, Int56, Int64, Uint8, Uint16, Uint24, Uint32, Uint40,
    Uint48, Uint56, Uint64,
};
pub use self::channel_list::ChannelList;
pub use self::channels_field::ChannelsField;
pub use self::composite::{OctStr, String};
pub use self::configuration_bitmask::ConfigurationBitmask;
pub use self::discrete::{
    Bool, Data8, Data16, Data24, Data32, Data40, Data48, Data56, Data64, Date, TimeOfDay,
    TryFromNaiveDateError, TryFromNaiveTimeError, TryIntoNaiveDateError, TryIntoNaiveTimeError,
    UtcTime,
};
pub use self::null::{NoData, Unknown};

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, ReprDiscriminant)]
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
    IeeeAddress(MacAddr8) = 0xf0,
    /// 128-bit Key.
    Key128([u8; 16]) = 0xf1,
}

impl FromLeStreamTagged for Type {
    type Tag = u8;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            0xff => Ok(Some(Self::Unknown)),
            0x00 => Ok(Some(Self::NoData)),
            0x08 => Ok(<[u8; 1]>::from_le_stream(bytes).map(Self::Data8)),
            0x09 => Ok(<[u8; 2]>::from_le_stream(bytes).map(Self::Data16)),
            0x0a => Ok(<[u8; 3]>::from_le_stream(bytes).map(Self::Data24)),
            0x0b => Ok(<[u8; 4]>::from_le_stream(bytes).map(Self::Data32)),
            0x0c => Ok(<[u8; 5]>::from_le_stream(bytes).map(Self::Data40)),
            0x0d => Ok(<[u8; 6]>::from_le_stream(bytes).map(Self::Data48)),
            0x0e => Ok(<[u8; 7]>::from_le_stream(bytes).map(Self::Data56)),
            0x0f => Ok(<[u8; 8]>::from_le_stream(bytes).map(Self::Data64)),
            0x10 => Ok(Bool::from_le_stream(bytes).map(Self::Boolean)),
            0x18 => Ok(u8::from_le_stream(bytes).map(Self::Map8)),
            0x19 => Ok(u16::from_le_stream(bytes).map(Self::Map16)),
            0x1b => Ok(u32::from_le_stream(bytes).map(Self::Map32)),
            0x1c => Ok(u64::from_le_stream(bytes).map(Self::Map64)),
            0x20 => Ok(Uint8::from_le_stream(bytes).map(Self::Uint8)),
            0x21 => Ok(Uint16::from_le_stream(bytes).map(Self::Uint16)),
            0x22 => Ok(Uint24::from_le_stream(bytes).map(Self::Uint24)),
            0x23 => Ok(Uint32::from_le_stream(bytes).map(Self::Uint32)),
            0x24 => Ok(Uint40::from_le_stream(bytes).map(Self::Uint40)),
            0x25 => Ok(Uint48::from_le_stream(bytes).map(Self::Uint48)),
            0x26 => Ok(Uint56::from_le_stream(bytes).map(Self::Uint56)),
            0x27 => Ok(Uint64::from_le_stream(bytes).map(Self::Uint64)),
            0x28 => Ok(Int8::from_le_stream(bytes).map(Self::Int8)),
            0x29 => Ok(Int16::from_le_stream(bytes).map(Self::Int16)),
            0x2a => Ok(Int24::from_le_stream(bytes).map(Self::Int24)),
            0x2b => Ok(Int32::from_le_stream(bytes).map(Self::Int32)),
            0x2c => Ok(Int40::from_le_stream(bytes).map(Self::Int40)),
            0x2d => Ok(Int48::from_le_stream(bytes).map(Self::Int48)),
            0x2e => Ok(Int56::from_le_stream(bytes).map(Self::Int56)),
            0x2f => Ok(Int64::from_le_stream(bytes).map(Self::Int64)),
            0x41 => Ok(OctStr::from_le_stream(bytes).map(Self::OctetString)),
            0x42 => Ok(String::from_le_stream(bytes).map(Self::String)),
            0xe0 => Ok(TimeOfDay::from_le_stream(bytes).map(Self::TimeOfDay)),
            0xe1 => Ok(Date::from_le_stream(bytes).map(Self::Date)),
            0xe2 => Ok(UtcTime::from_le_stream(bytes).map(Self::UtcTime)),
            0xe8 => Ok(u16::from_le_stream(bytes).map(Self::ClusterId)),
            0xe9 => Ok(u16::from_le_stream(bytes).map(Self::AttributeId)),
            0xea => Ok(u32::from_le_stream(bytes).map(Self::BacnetObjectId)),
            0xf0 => Ok(MacAddr8::from_le_stream(bytes).map(Self::IeeeAddress)),
            0xf1 => Ok(<[u8; 16]>::from_le_stream(bytes).map(Self::Key128)),
            other => Err(other),
        }
    }
}

impl ToLeStream for Type {
    type Iter = IntoIter<u8>;

    fn to_le_stream(self) -> Self::Iter {
        let mut bytes = Vec::new();
        bytes.extend(self.discriminant().to_le_stream());

        #[expect(clippy::match_same_arms)]
        match self {
            Self::Unknown => {}
            Self::NoData => {}
            Self::Data8(value) => bytes.extend(value.to_le_stream()),
            Self::Data16(value) => bytes.extend(value.to_le_stream()),
            Self::Data24(value) => bytes.extend(value.to_le_stream()),
            Self::Data32(value) => bytes.extend(value.to_le_stream()),
            Self::Data40(value) => bytes.extend(value.to_le_stream()),
            Self::Data48(value) => bytes.extend(value.to_le_stream()),
            Self::Data56(value) => bytes.extend(value.to_le_stream()),
            Self::Data64(value) => bytes.extend(value.to_le_stream()),
            Self::Boolean(value) => bytes.extend(value.to_le_stream()),
            Self::Map8(value) => bytes.extend(value.to_le_stream()),
            Self::Map16(value) => bytes.extend(value.to_le_stream()),
            Self::Map32(value) => bytes.extend(value.to_le_stream()),
            Self::Map64(value) => bytes.extend(value.to_le_stream()),
            Self::Uint8(value) => bytes.extend(value.to_le_stream()),
            Self::Uint16(value) => bytes.extend(value.to_le_stream()),
            Self::Uint24(value) => bytes.extend(value.to_le_stream()),
            Self::Uint32(value) => bytes.extend(value.to_le_stream()),
            Self::Uint40(value) => bytes.extend(value.to_le_stream()),
            Self::Uint48(value) => bytes.extend(value.to_le_stream()),
            Self::Uint56(value) => bytes.extend(value.to_le_stream()),
            Self::Uint64(value) => bytes.extend(value.to_le_stream()),
            Self::Int8(value) => bytes.extend(value.to_le_stream()),
            Self::Int16(value) => bytes.extend(value.to_le_stream()),
            Self::Int24(value) => bytes.extend(value.to_le_stream()),
            Self::Int32(value) => bytes.extend(value.to_le_stream()),
            Self::Int40(value) => bytes.extend(value.to_le_stream()),
            Self::Int48(value) => bytes.extend(value.to_le_stream()),
            Self::Int56(value) => bytes.extend(value.to_le_stream()),
            Self::Int64(value) => bytes.extend(value.to_le_stream()),
            Self::OctetString(value) => bytes.extend(value.to_le_stream()),
            Self::String(value) => bytes.extend(value.to_le_stream()),
            Self::TimeOfDay(value) => bytes.extend(value.to_le_stream()),
            Self::Date(value) => bytes.extend(value.to_le_stream()),
            Self::UtcTime(value) => bytes.extend(value.to_le_stream()),
            Self::ClusterId(value) => bytes.extend(value.to_le_stream()),
            Self::AttributeId(value) => bytes.extend(value.to_le_stream()),
            Self::BacnetObjectId(value) => bytes.extend(value.to_le_stream()),
            Self::IeeeAddress(value) => bytes.extend(value.to_le_stream()),
            Self::Key128(value) => bytes.extend(value.to_le_stream()),
        }

        bytes.into_iter()
    }
}
