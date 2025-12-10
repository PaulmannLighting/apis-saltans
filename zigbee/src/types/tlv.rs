use le_stream::{FromLeStream, ToLeStream};

pub use self::global::Global;
pub use self::tag::Tag;

mod global;
mod tag;

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
        let tag = u8::from_le_stream(&mut bytes)?;

        #[expect(clippy::unwrap_in_result)]
        let len = u8::from_le_stream(&mut bytes)
            .map(usize::from)?
            .checked_add(1)
            .expect("u8::MAX + 1 cannot overflow usize");
        let buffer = bytes.take(len).collect::<Vec<_>>();

        if buffer.len() < len {
            return None;
        }

        let bytes = buffer.into_iter();

        match tag {
            0..=63 => todo!("Parse local TLV tags"),
            64..=255 => Global::from_le_stream_with_tag(tag, bytes)
                .map(Self::Global)
                .ok(),
        }
    }
}
