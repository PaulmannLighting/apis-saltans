// TODO: Is this the correct representation for BitmapDiscrete?
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Bitmap {
    Map8(u8),
    Map16(u16),
    Map24(u32),
    Map32(u32),
    Map40(u64),
    Map48(u64),
    Map56(u64),
    Map64(u64),
}
