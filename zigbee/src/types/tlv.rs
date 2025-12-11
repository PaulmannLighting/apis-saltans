//! Type-Length-Value (TLV) encoded structures for Zigbee.

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

pub use self::encapsulated_global::EncapsulatedGlobal;
pub use self::global::{
    BeaconAppendixEncapsulation, DeviceCapabilityExtension, FragmentationOptions, Global,
    JoinerEncapsulation, KeyNegotiationProtocols, ManufacturerSpecific, NextChannelChange,
    NextPanIdChange, PanIdConflictReport, PreSharedSecrets, RouterInformation,
    SupportedKeyNegotiation, SymmetricPassphrase,
};
use self::iter::TlvLeStream;
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
        match u8::from_le_stream(&mut bytes)? {
            tag @ 0..=63 => L::from_le_stream_tagged(tag, bytes).ok()?.map(Self::Local),
            tag @ 64..=255 => G::from_le_stream_tagged(tag, bytes).ok()?.map(Self::Global),
        }
    }
}

impl<L, G> ToLeStream for Tlv<L, G>
where
    L: ToLeStream,
    G: ToLeStream,
{
    type Iter = TlvLeStream<L::Iter, G::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Local(local) => TlvLeStream::Local(local.to_le_stream()),
            Self::Global(global) => TlvLeStream::Global(global.to_le_stream()),
        }
    }
}

mod iter {
    pub enum TlvLeStream<L, G> {
        Local(L),
        Global(G),
    }

    impl<L, G> Iterator for TlvLeStream<L, G>
    where
        L: Iterator<Item = u8>,
        G: Iterator<Item = u8>,
    {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::Local(iter) => iter.next(),
                Self::Global(iter) => iter.next(),
            }
        }
    }
}
