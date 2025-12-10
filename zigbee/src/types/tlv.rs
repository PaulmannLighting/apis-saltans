//! Type-Length-Value (TLV) encoded structures for Zigbee.

use le_stream::{FromLeStream, FromLeStreamTagged};

pub use self::encapsulated_global::EncapsulatedGlobal;
pub use self::global::{
    BeaconAppendixEncapsulation, DeviceCapabilityExtension, FragmentationOptions, Global,
    JoinerEncapsulation, KeyNegotiationProtocols, ManufacturerSpecific, NextChannelChange,
    NextPanIdChange, PanIdConflictReport, PreSharedSecrets, RouterInformation,
    SupportedKeyNegotiation, SymmetricPassphrase,
};
pub use self::local::Local;
pub use self::tag::Tag;

mod encapsulated_global;
mod global;
mod local;
mod tag;

/// A Type-Length-Value (TLV) encoded structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Tlv<L = Local, G = Global> {
    /// Local TLV tags.
    Local(L),
    /// Global TLV tags.
    Global(G),
}

impl<L, G> FromLeStream for Tlv<L, G>
where
    L: FromLeStreamTagged<Tag = u8>,
    G: FromLeStreamTagged<Tag = u8>,
{
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
            0..=63 => L::from_le_stream_tagged(tag, bytes).ok()?.map(Self::Local),
            64..=255 => G::from_le_stream_tagged(tag, bytes).ok()?.map(Self::Global),
        }
    }
}
