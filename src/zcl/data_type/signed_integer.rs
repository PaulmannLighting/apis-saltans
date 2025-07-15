#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignedInteger {
    Int8(i8),
    Int16(i16),
    Int24(i32),
    Int32(i32),
    Int40(i64),
    Int48(i64),
    Int56(i64),
    Int64(i64),
}
