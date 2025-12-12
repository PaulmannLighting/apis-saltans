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
pub use self::local::{ClearAllBindingsReqEui64, Local};
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

#[cfg(test)]
mod tests {
    use le_stream::ToLeStream;
    use macaddr::MacAddr8;

    use super::*;

    const SUPPORTED_KEY_NEGOTIATION_METHODS: Tlv = Tlv::Global(
        Global::SupportedKeyNegotiationMethods(SupportedKeyNegotiation::new(
            KeyNegotiationProtocols::STATIC_KEY_REQUEST,
            PreSharedSecrets::INSTALL_CODE_KEY,
            None,
        )),
    );
    const SUPPORTED_KEY_NEGOTIATION_METHODS_WITH_SOURCE_DEVICE_EUI64: Tlv = Tlv::Global(
        Global::SupportedKeyNegotiationMethods(SupportedKeyNegotiation::new(
            KeyNegotiationProtocols::STATIC_KEY_REQUEST,
            PreSharedSecrets::INSTALL_CODE_KEY,
            Some(MacAddr8::new(1, 2, 3, 4, 5, 6, 7, 8)),
        )),
    );

    #[test]
    fn supported_key_negotiation_methods_to_le_stream() {
        let bytes: Vec<_> = SUPPORTED_KEY_NEGOTIATION_METHODS.to_le_stream().collect();
        assert_eq!(bytes, vec![65, 1, 0b1000_0000, 0b0100_0000]);
    }

    #[test]
    fn supported_key_negotiation_methods_with_source_device_eui64_to_le_stream() {
        let bytes: Vec<_> = SUPPORTED_KEY_NEGOTIATION_METHODS_WITH_SOURCE_DEVICE_EUI64
            .to_le_stream()
            .collect();
        assert_eq!(
            bytes,
            vec![65, 9, 0b1000_0000, 0b0100_0000, 8, 7, 6, 5, 4, 3, 2, 1]
        );
    }
    #[test]
    fn supported_key_negotiation_methods_from_le_stream() {
        let bytes = vec![65, 1, 0b1000_0000, 0b0100_0000];
        let tlv: Option<Tlv> = Tlv::from_le_stream(bytes.into_iter());
        assert_eq!(tlv, Some(SUPPORTED_KEY_NEGOTIATION_METHODS));
    }

    #[test]
    fn supported_key_negotiation_methods_with_source_device_eui64_from_le_stream() {
        let bytes = vec![65, 9, 0b1000_0000, 0b0100_0000, 8, 7, 6, 5, 4, 3, 2, 1];
        let tlv: Option<Tlv> = Tlv::from_le_stream(bytes.into_iter());
        assert_eq!(
            tlv,
            Some(SUPPORTED_KEY_NEGOTIATION_METHODS_WITH_SOURCE_DEVICE_EUI64)
        );
    }
}
