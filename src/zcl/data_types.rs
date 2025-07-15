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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DataType {
    Null,
    /// Generic
    Data8(u8),
    Data16([u8; 2]),
    Data24([u8; 3]),
    Data32([u8; 4]),
    Data40([u8; 5]),
    Data48([u8; 6]),
    Data56([u8; 7]),
    Data64([u8; 8]),
    Logical(bool),
    /// Bitmap
    Map8(u8),
    Map16(u16),
    Map24(u32),
    Map32(u32),
    Map40(u64),
    Map48(u64),
    Map56(u64),
    Map64(u64),
    /// Unsigned integer
    Uint8(u8),
    Uint16(u16),
    Uint24(u32),
    Uint32(u32),
    Uint40(u64),
    Uint48(u64),
    Uint56(u64),
    Uint64(u64),
    /// Signed integer
    Int8(i8),
    Int16(i16),
    Int24(i32),
    Int32(i32),
    Int40(i64),
    Int48(i64),
    Int56(i64),
    Int64(i64),
    /// Enumeration
    Enum8(u8),
    Enum16(u16),
    /// Floating point
    Semi(f16),
    Single(f32),
    Double(f64),
    /// String
    OctStr(Vec<u8>),
    String(String),
    OctStr16(Vec<u8>),
    String16(String),
    /// Ordered sequence
    Array(Array),
    Structure(Structure),
    /// Collection
    Set(Vec<u8>),
    Bag(Vec<u8>),
    /// Time
    TimeOfDay(TimeOfDay),
    Date(Date),
    UtcTime(UtcTime),
    /// Identifier
    ClusterId(u16),
    AttributeId(u16),
    BacNetOid(u32),
    /// Miscellaneous
    IeeeAddress(MacAddr8),
    SecurityKey([u8; 16]),
    Opaque(Vec<u8>),
    Unknown,
}
