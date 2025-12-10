use le_stream::FromLeStream;

pub use self::manufacturer_specific::ManufacturerSpecific;
pub use self::next_pan_id_change::NextPanIdChange;
pub use self::pan_id_conflict_report::PanIdConflictReport;
pub use self::supported_key_negotiation::{
    KeyNegotiationProtocols, PreSharedSecrets, SupportedKeyNegotiation,
};
use super::Tag;

mod manufacturer_specific;
mod next_pan_id_change;
mod pan_id_conflict_report;
mod supported_key_negotiation;

#[derive(Clone, Debug)]
pub enum Global {
    /// Manufacturer Specific TLV.
    ManufacturerSpecific(ManufacturerSpecific),
    /// Supported Key Negotiation TLV.
    SupportedKeyNegotiationMethods(SupportedKeyNegotiation),
    /// Pan ID Conflict Report TLV.
    PanIdConflictReport(PanIdConflictReport),
    /// Next PAN ID Change TLV.
    NextPanIdChange(NextPanIdChange),
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
    pub(crate) fn from_le_stream_with_tag<T>(tag: u8, mut bytes: T) -> le_stream::Result<Self>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            ManufacturerSpecific::TAG => {
                ManufacturerSpecific::from_le_stream_exact(bytes).map(Self::ManufacturerSpecific)
            }
            SupportedKeyNegotiation::TAG => SupportedKeyNegotiation::from_le_stream_exact(bytes)
                .map(Self::SupportedKeyNegotiationMethods),
            PanIdConflictReport::TAG => {
                PanIdConflictReport::from_le_stream_exact(bytes).map(Self::PanIdConflictReport)
            }
            NextPanIdChange::TAG => {
                NextPanIdChange::from_le_stream_exact(bytes).map(Self::NextPanIdChange)
            }
            _ => todo!("Implement and parse other TLVs"),
        }
    }
}
