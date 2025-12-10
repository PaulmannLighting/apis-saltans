use le_stream::FromLeStream;

pub use self::manufacturer_specific::ManufacturerSpecific;
pub use self::supported_key_negotiation::{
    KeyNegotiationProtocols, PreSharedSecrets, SupportedKeyNegotiation,
};
use super::tlv::Tlv;
use crate::types::tlv::global::pan_id_conflict_report::PanIdConflictReport;

mod manufacturer_specific;
mod pan_id_conflict_report;
mod supported_key_negotiation;

#[derive(Clone, Debug)]
pub enum Global {
    /// Manufacturer Specific TLV.
    ManufacturerSpecific(ManufacturerSpecific),
    /// Supported Key Negotiation TLV.
    SupportedKeyNegotiationMethods(SupportedKeyNegotiation),
    PanIdConflictReport(PanIdConflictReport),
    NextPanId,
    NextChannelChange,
    SymmetricPassphrase,
    RouterInformation,
    FragmentationParameters,
    JoinerEncapsulation,
    BeaconAppendixEncapsulation,
    BdbEncapsulation,
    ConfigurationParameters,
    DeviceCapabilityExtension,
}

impl Global {
    /// Parse a Global TLV from a byte stream with a given tag.
    pub(crate) fn from_le_stream_with_tag<T>(tag: u8, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
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
            ManufacturerSpecific::TAG => {
                ManufacturerSpecific::from_le_stream(bytes).map(Self::ManufacturerSpecific)
            }
            SupportedKeyNegotiation::TAG => SupportedKeyNegotiation::from_le_stream(bytes)
                .map(Self::SupportedKeyNegotiationMethods),
            _ => todo!("Implement and parse other TLVs"),
        }
    }
}
