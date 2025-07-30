use alloc::vec::Vec;

use array::Array;
use date::Date;
use half::f16;
use intx::{I24, I40, I48, I56, U24, U40, U48, U56};
use macaddr::MacAddr8;
use structure::Structure;
use time_of_day::TimeOfDay;
use utc_time::UtcTime;

use crate::types::{OctStr, OctStr16, String, String16};

mod array;
mod date;
mod structure;
mod time_of_day;
mod utc_time;

/// Available Zigbee data types.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum DataType {
    /// No data.
    Null = 0x00,
    // Generic
    /// 8-bit data.
    Data8(u8) = 0x08,
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
    // Boolean
    /// Boolean.
    Logical(bool) = 0x10,
    // Bitmap
    /// 8-bit bitmap.
    Map8(u8) = 0x18,
    /// 16-bit bitmap.
    Map16(u16) = 0x19,
    /// 24-bit bitmap.
    Map24(U24) = 0x1a,
    /// 32-bit bitmap.
    Map32(u32) = 0x1b,
    /// 40-bit bitmap.
    Map40(U40) = 0x1c,
    /// 48-bit bitmap.
    Map48(U48) = 0x1d,
    /// 56-bit bitmap.
    Map56(U56) = 0x1e,
    /// 64-bit bitmap.
    Map64(u64) = 0x1f,
    // Unsigned integer
    /// 8-bit unsigned integer.
    Uint8(u8) = 0x20,
    /// 16-bit unsigned integer.
    Uint16(u16) = 0x21,
    /// 24-bit unsigned integer.
    Uint24(U24) = 0x22,
    /// 32-bit unsigned integer.
    Uint32(u32) = 0x23,
    /// 40-bit unsigned integer.
    Uint40(U40) = 0x24,
    /// 48-bit unsigned integer.
    Uint48(U48) = 0x25,
    /// 56-bit unsigned integer.
    Uint56(U56) = 0x26,
    /// 64-bit unsigned integer.
    Uint64(u64) = 0x27,
    // Signed integer
    /// 8-bit signed integer.
    Int8(i8) = 0x28,
    /// 16-bit signed integer.
    Int16(i16) = 0x29,
    /// 24-bit signed integer.
    Int24(I24) = 0x2a,
    /// 32-bit signed integer.
    Int32(i32) = 0x2b,
    /// 40-bit signed integer.
    Int40(I40) = 0x2c,
    /// 48-bit signed integer.
    Int48(I48) = 0x2d,
    /// 56-bit signed integer.
    Int56(I56) = 0x2e,
    /// 64-bit signed integer.
    Int64(i64) = 0x2f,
    // Enumeration
    /// 8-bit enumeration.
    Enum8(u8) = 0x30,
    /// 16-bit enumeration.
    Enum16(u16) = 0x31,
    // Floating point
    /// 16-bit floating point.
    Semi(f16) = 0x38,
    /// 32-bit floating point.
    Single(f32) = 0x39,
    /// 64-bit floating point.
    Double(f64) = 0x3a,
    // String
    /// Octet string.
    OctStr(OctStr) = 0x41,
    /// Character string.
    String(String) = 0x42,
    /// Long octet string.
    OctStr16(OctStr16) = 0x43,
    /// Long character string.
    String16(String16) = 0x44,
    // Ordered sequence
    /// Array.
    Array(Array) = 0x48,
    /// Structure.
    Structure(Structure) = 0x4c,
    // Collection
    /// Set.
    ///
    /// TODO: Choose a more appropriate type for the set.
    Set(Vec<u8>) = 0x50,
    /// Bag.
    ///
    /// TODO: Choose a more appropriate type for the bag.
    Bag(Vec<u8>) = 0x51,
    // Time
    /// Time of day.
    TimeOfDay(TimeOfDay) = 0xe0,
    /// Date.
    Date(Date) = 0xe1,
    /// UTC time.
    UtcTime(UtcTime) = 0xe2,
    // Identifier
    /// Cluster ID.
    ClusterId(u16) = 0xe8,
    /// Attribute ID.
    AttributeId(u16) = 0xe9,
    /// `BACnet` OID.
    BacNetOid(u32) = 0xea,
    // Miscellaneous
    /// IEEE address.
    IeeeAddress(MacAddr8) = 0xf0,
    /// 128-bit security key.
    SecurityKey([u8; 16]) = 0xf1,
    /// Opaque data.
    Opaque(Vec<u8>) = 0xf2,
    /// Unknown data type.
    Unknown = 0xff,
}
