use array::Array;
use date::Date;
use half::f16;
use macaddr::MacAddr8;
use structure::Structure;
use time_of_day::TimeOfDay;
use utc_time::UtcTime;

mod array;
mod date;
mod structure;
mod time_of_day;
mod utc_time;

/// Available Zigbee data types.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum DataType {
    Null = 0x00,
    // Generic
    Data8(u8) = 0x08,
    Data16([u8; 2]) = 0x09,
    Data24([u8; 3]) = 0x0a,
    Data32([u8; 4]) = 0x0b,
    Data40([u8; 5]) = 0x0c,
    Data48([u8; 6]) = 0x0d,
    Data56([u8; 7]) = 0x0e,
    Data64([u8; 8]) = 0x0f,
    Logical(bool) = 0x10,
    // Bitmap
    Map8(u8) = 0x18,
    Map16(u16) = 0x19,
    Map24(u32) = 0x1a,
    Map32(u32) = 0x1b,
    Map40(u64) = 0x1c,
    Map48(u64) = 0x1d,
    Map56(u64) = 0x1e,
    Map64(u64) = 0x1f,
    // Unsigned integer
    Uint8(u8) = 0x20,
    Uint16(u16) = 0x21,
    Uint24(u32) = 0x22,
    Uint32(u32) = 0x23,
    Uint40(u64) = 0x24,
    Uint48(u64) = 0x25,
    Uint56(u64) = 0x26,
    Uint64(u64) = 0x27,
    // Signed integer
    Int8(i8) = 0x28,
    Int16(i16) = 0x29,
    Int24(i32) = 0x2a,
    Int32(i32) = 0x2b,
    Int40(i64) = 0x2c,
    Int48(i64) = 0x2d,
    Int56(i64) = 0x2e,
    Int64(i64) = 0x2f,
    // Enumeration
    Enum8(u8) = 0x30,
    Enum16(u16) = 0x31,
    // Floating point
    Semi(f16) = 0x38,
    Single(f32) = 0x39,
    Double(f64) = 0x3a,
    // String
    OctStr(Vec<u8>) = 0x41,
    String(String) = 0x42,
    OctStr16(Vec<u8>) = 0x43,
    String16(String) = 0x44,
    // Ordered sequence
    Array(Array) = 0x48,
    Structure(Structure) = 0x4c,
    // Collection
    Set(Vec<u8>) = 0x50,
    Bag(Vec<u8>) = 0x51,
    // Time
    TimeOfDay(TimeOfDay) = 0xe0,
    Date(Date) = 0xe1,
    UtcTime(UtcTime) = 0xe2,
    // Identifier
    ClusterId(u16) = 0xe8,
    AttributeId(u16) = 0xe9,
    BacNetOid(u32) = 0xea,
    // Miscellaneous
    IeeeAddress(MacAddr8) = 0xf0,
    SecurityKey([u8; 16]) = 0xf1,
    Opaque(Vec<u8>) = 0xf2,
    Unknown = 0xff,
}
