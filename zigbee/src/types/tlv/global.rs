use le_stream::{FromLeStream, FromLeStreamTagged};

pub use self::fragmentation_parameters::{FragmentationOptions, FragmentationParameters};
pub use self::joiner_encapsulation::JoinerEncapsulation;
pub use self::manufacturer_specific::ManufacturerSpecific;
pub use self::next_channel_change::NextChannelChange;
pub use self::next_pan_id_change::NextPanIdChange;
pub use self::pan_id_conflict_report::PanIdConflictReport;
pub use self::router_information::RouterInformation;
pub use self::supported_key_negotiation::{
    KeyNegotiationProtocols, PreSharedSecrets, SupportedKeyNegotiation,
};
pub use self::symmetric_passphrase::SymmetricPassphrase;
use super::Tag;
use crate::types::tlv::global::beacon_appendix_encapsulation::BeaconAppendixEncapsulation;

mod beacon_appendix_encapsulation;
mod fragmentation_parameters;
mod joiner_encapsulation;
mod manufacturer_specific;
mod next_channel_change;
mod next_pan_id_change;
mod pan_id_conflict_report;
mod router_information;
mod supported_key_negotiation;
mod symmetric_passphrase;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Global {
    /// Manufacturer Specific TLV.
    ManufacturerSpecific(ManufacturerSpecific),
    /// Supported Key Negotiation TLV.
    SupportedKeyNegotiationMethods(SupportedKeyNegotiation),
    /// Pan ID Conflict Report TLV.
    PanIdConflictReport(PanIdConflictReport),
    /// Next PAN ID Change TLV.
    NextPanIdChange(NextPanIdChange),
    /// Next Channel Change TLV.
    NextChannelChange(NextChannelChange),
    /// Symmetric Passphrase TLV.
    SymmetricPassphrase(SymmetricPassphrase),
    /// Router Information TLV.
    RouterInformation(RouterInformation),
    /// Fragmentation Parameters TLV.
    FragmentationParameters(FragmentationParameters),
    /// Joiner Encapsulation TLV.
    JoinerEncapsulation(JoinerEncapsulation),
    /// Beacon Appendix Encapsulation TLV.
    BeaconAppendixEncapsulation(BeaconAppendixEncapsulation),
    BdbEncapsulation,
    ConfigurationParameters,
    DeviceCapabilityExtension,
}

impl FromLeStreamTagged for Global {
    type Tag = u8;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            ManufacturerSpecific::TAG => Ok(ManufacturerSpecific::from_le_stream_exact(bytes)
                .map(Self::ManufacturerSpecific)
                .ok()),
            SupportedKeyNegotiation::TAG => {
                Ok(SupportedKeyNegotiation::from_le_stream_exact(bytes)
                    .map(Self::SupportedKeyNegotiationMethods)
                    .ok())
            }
            PanIdConflictReport::TAG => Ok(PanIdConflictReport::from_le_stream_exact(bytes)
                .map(Self::PanIdConflictReport)
                .ok()),
            NextPanIdChange::TAG => Ok(NextPanIdChange::from_le_stream_exact(bytes)
                .map(Self::NextPanIdChange)
                .ok()),
            NextChannelChange::TAG => Ok(NextChannelChange::from_le_stream_exact(bytes)
                .map(Self::NextChannelChange)
                .ok()),
            SymmetricPassphrase::TAG => Ok(SymmetricPassphrase::from_le_stream_exact(bytes)
                .map(Self::SymmetricPassphrase)
                .ok()),
            RouterInformation::TAG => Ok(RouterInformation::from_le_stream_exact(bytes)
                .map(Self::RouterInformation)
                .ok()),
            FragmentationParameters::TAG => {
                Ok(FragmentationParameters::from_le_stream_exact(bytes)
                    .map(Self::FragmentationParameters)
                    .ok())
            }
            JoinerEncapsulation::TAG => Ok(JoinerEncapsulation::from_le_stream_exact(bytes)
                .map(Self::JoinerEncapsulation)
                .ok()),
            BeaconAppendixEncapsulation::TAG => {
                Ok(BeaconAppendixEncapsulation::from_le_stream_exact(bytes)
                    .map(Self::BeaconAppendixEncapsulation)
                    .ok())
            }
            _ => Err(tag),
        }
    }
}
