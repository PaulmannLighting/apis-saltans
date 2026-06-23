//! Type-Length-Value (TLV) encoded structures for Zigbee.

use le_stream::{FromLeStream, ToLeStream};

pub use self::encapsulated_global::EncapsulatedGlobal;
pub use self::general::{General, Payload};
pub use self::global::{
    BeaconAppendixEncapsulation, DeviceCapabilityExtension, FragmentationOptions, Global,
    JoinerEncapsulation, KeyNegotiationProtocols, ManufacturerSpecific, NextChannelChange,
    NextPanIdChange, PanIdConflictReport, PreSharedSecrets, RouterInformation,
    SupportedKeyNegotiation, SymmetricPassphrase,
};
pub use self::local::{ClearAllBindingsReqEui64, Local};
pub use self::tag::Tag;

mod encapsulated_global;
mod general;
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

impl<L, G> From<Tlv<L, G>> for General
where
    L: Into<Self>,
    G: Into<Self>,
{
    fn from(tlv: Tlv<L, G>) -> Self {
        match tlv {
            Tlv::Local(local) => local.into(),
            Tlv::Global(global) => global.into(),
        }
    }
}

impl<L, G> TryFrom<General> for Tlv<L, G>
where
    L: TryFrom<General, Error = u8>,
    G: TryFrom<General, Error = u8>,
{
    type Error = u8;

    fn try_from(general: General) -> Result<Self, Self::Error> {
        match general.typ() {
            0..=63 => L::try_from(general).map(Self::Local),
            64..=255 => G::try_from(general).map(Self::Global),
        }
    }
}

impl<L, G> FromLeStream for Tlv<L, G>
where
    L: TryFrom<General, Error = u8>,
    G: TryFrom<General, Error = u8>,
{
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        General::from_le_stream(bytes)?.try_into().ok()
    }
}

impl<L, G> ToLeStream for Tlv<L, G>
where
    L: Into<General>,
    G: Into<General>,
{
    type Iter = <General as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        <Self as Into<General>>::into(self).to_le_stream()
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use alloc::vec::Vec;

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
