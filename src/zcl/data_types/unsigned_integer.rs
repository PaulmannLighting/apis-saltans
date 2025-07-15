#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnsignedInteger {
    Uint8(u8),
    Uint16(u16),
    Uint24(u32),
    Uint32(u32),
    Uint40(u64),
    Uint48(u64),
    Uint56(u64),
    Uint64(u64),
}
