use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::global::Global;

mod global;
mod tlv;

/// A Type-Length-Value (TLV) encoded structure.
#[derive(Clone, Debug)]
pub enum Tlv {
    /// Local TLV tags.
    Local,
    /// Global TLV tags.
    Global(Global),
}

impl FromLeStream for Tlv {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        match u8::from_le_stream(&mut bytes)? {
            0..=63 => todo!("Parse local TLV tags"),
            tag @ 64..=255 => Global::from_le_stream_with_tag(tag, bytes).map(Self::Global),
        }
    }
}
