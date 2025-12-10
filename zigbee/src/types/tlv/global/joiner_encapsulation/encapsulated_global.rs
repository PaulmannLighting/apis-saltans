use le_stream::{FromLeStream, FromLeStreamTagged};

use crate::types::tlv::Tag;
use crate::types::tlv::global::{
    FragmentationParameters, ManufacturerSpecific, NextChannelChange, NextPanIdChange,
    PanIdConflictReport, RouterInformation, SupportedKeyNegotiation, SymmetricPassphrase,
};

/// Encapsulated Global TLV types.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EncapsulatedGlobal {
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
    ConfigurationParameters,
    DeviceCapabilityExtension,
}

impl FromLeStreamTagged for EncapsulatedGlobal {
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
            _ => Err(tag),
        }
    }
}
