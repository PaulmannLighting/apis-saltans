use intx::{I24, I40, I48, I56, U24, U40, U48, U56};
use le_stream::ToLeStream;
use macaddr::MacAddr8;

use crate::types::{Bool, Date, OctStr, String, TimeOfDay, Type, UtcTime};

#[derive(Debug)]
pub enum TypeIter {
    Empty,
    Data8(<[u8; 1] as ToLeStream>::Iter),
    Data16(<[u8; 2] as ToLeStream>::Iter),
    Data24(<[u8; 3] as ToLeStream>::Iter),
    Data32(<[u8; 4] as ToLeStream>::Iter),
    Data40(<[u8; 5] as ToLeStream>::Iter),
    Data48(<[u8; 6] as ToLeStream>::Iter),
    Data56(<[u8; 7] as ToLeStream>::Iter),
    Data64(<[u8; 8] as ToLeStream>::Iter),
    Boolean(<Bool as ToLeStream>::Iter),
    Uint8(<u8 as ToLeStream>::Iter),
    Uint16(<u16 as ToLeStream>::Iter),
    Uint24(<U24 as ToLeStream>::Iter),
    Uint32(<u32 as ToLeStream>::Iter),
    Uint40(<U40 as ToLeStream>::Iter),
    Uint48(<U48 as ToLeStream>::Iter),
    Uint56(<U56 as ToLeStream>::Iter),
    Uint64(<u64 as ToLeStream>::Iter),
    Int8(<i8 as ToLeStream>::Iter),
    Int16(<i16 as ToLeStream>::Iter),
    Int24(<I24 as ToLeStream>::Iter),
    Int32(<i32 as ToLeStream>::Iter),
    Int40(<I40 as ToLeStream>::Iter),
    Int48(<I48 as ToLeStream>::Iter),
    Int56(<I56 as ToLeStream>::Iter),
    Int64(<i64 as ToLeStream>::Iter),
    OctetString(<OctStr as ToLeStream>::Iter),
    String(<String as ToLeStream>::Iter),
    TimeOfDay(<TimeOfDay as ToLeStream>::Iter),
    Date(<Date as ToLeStream>::Iter),
    UtcTime(<UtcTime as ToLeStream>::Iter),
    IeeeAddress(<MacAddr8 as ToLeStream>::Iter),
    Key128(<[u8; 16] as ToLeStream>::Iter),
}

impl From<Type> for TypeIter {
    fn from(t: Type) -> Self {
        #[expect(clippy::match_same_arms)]
        match t {
            Type::NoData | Type::Unknown => Self::Empty,
            Type::Data8(v) => Self::Data8(v.to_le_stream()),
            Type::Data16(v) => Self::Data16(v.to_le_stream()),
            Type::Data24(v) => Self::Data24(v.to_le_stream()),
            Type::Data32(v) => Self::Data32(v.to_le_stream()),
            Type::Data40(v) => Self::Data40(v.to_le_stream()),
            Type::Data48(v) => Self::Data48(v.to_le_stream()),
            Type::Data56(v) => Self::Data56(v.to_le_stream()),
            Type::Data64(v) => Self::Data64(v.to_le_stream()),
            Type::Boolean(v) => Self::Boolean(v.to_le_stream()),
            Type::Map8(v) => Self::Uint8(v.to_le_stream()),
            Type::Map16(v) => Self::Uint16(v.to_le_stream()),
            Type::Map32(v) => Self::Uint32(v.to_le_stream()),
            Type::Map64(v) => Self::Uint64(v.to_le_stream()),
            Type::Uint8(v) => Self::Uint8(v.to_le_stream()),
            Type::Uint16(v) => Self::Uint16(v.to_le_stream()),
            Type::Uint24(v) => Self::Uint24(v.to_le_stream()),
            Type::Uint32(v) => Self::Uint32(v.to_le_stream()),
            Type::Uint40(v) => Self::Uint40(v.to_le_stream()),
            Type::Uint48(v) => Self::Uint48(v.to_le_stream()),
            Type::Uint56(v) => Self::Uint56(v.to_le_stream()),
            Type::Uint64(v) => Self::Uint64(v.to_le_stream()),
            Type::Int8(v) => Self::Int8(v.to_le_stream()),
            Type::Int16(v) => Self::Int16(v.to_le_stream()),
            Type::Int24(v) => Self::Int24(v.to_le_stream()),
            Type::Int32(v) => Self::Int32(v.to_le_stream()),
            Type::Int40(v) => Self::Int40(v.to_le_stream()),
            Type::Int48(v) => Self::Int48(v.to_le_stream()),
            Type::Int56(v) => Self::Int56(v.to_le_stream()),
            Type::Int64(v) => Self::Int64(v.to_le_stream()),
            Type::Enum8(v) => Self::Uint8(v.to_le_stream()),
            Type::Enum16(v) => Self::Uint16(v.to_le_stream()),
            Type::OctetString(v) => Self::OctetString(v.to_le_stream()),
            Type::String(v) => Self::String(v.to_le_stream()),
            Type::TimeOfDay(v) => Self::TimeOfDay(v.to_le_stream()),
            Type::Date(v) => Self::Date(v.to_le_stream()),
            Type::UtcTime(v) => Self::UtcTime(v.to_le_stream()),
            Type::ClusterId(v) => Self::Uint16(v.to_le_stream()),
            Type::AttributeId(v) => Self::Uint16(v.to_le_stream()),
            Type::BacnetObjectId(v) => Self::Uint32(v.to_le_stream()),
            Type::IeeeAddress(v) => Self::IeeeAddress(v.to_le_stream()),
            Type::Key128(v) => Self::Key128(v.to_le_stream()),
        }
    }
}

impl Iterator for TypeIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::Empty => None,
            Self::Data8(iter) => iter.next(),
            Self::Data16(iter) => iter.next(),
            Self::Data24(iter) => iter.next(),
            Self::Data32(iter) => iter.next(),
            Self::Data40(iter) => iter.next(),
            Self::Data48(iter) => iter.next(),
            Self::Data56(iter) => iter.next(),
            Self::Data64(iter) => iter.next(),
            Self::Boolean(iter) => iter.next(),
            Self::Uint8(iter) => iter.next(),
            Self::Uint16(iter) => iter.next(),
            Self::Uint24(iter) => iter.next(),
            Self::Uint32(iter) => iter.next(),
            Self::Uint40(iter) => iter.next(),
            Self::Uint48(iter) => iter.next(),
            Self::Uint56(iter) => iter.next(),
            Self::Uint64(iter) => iter.next(),
            Self::Int8(iter) => iter.next(),
            Self::Int16(iter) => iter.next(),
            Self::Int24(iter) => iter.next(),
            Self::Int32(iter) => iter.next(),
            Self::Int40(iter) => iter.next(),
            Self::Int48(iter) => iter.next(),
            Self::Int56(iter) => iter.next(),
            Self::Int64(iter) => iter.next(),
            Self::OctetString(iter) => iter.next(),
            Self::String(iter) => iter.next(),
            Self::TimeOfDay(iter) => iter.next(),
            Self::Date(iter) => iter.next(),
            Self::UtcTime(iter) => iter.next(),
            Self::IeeeAddress(iter) => iter.next(),
            Self::Key128(iter) => iter.next(),
        }
    }
}
