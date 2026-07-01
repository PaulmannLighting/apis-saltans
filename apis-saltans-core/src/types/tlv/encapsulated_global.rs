use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::global::{
    ConfigurationParameters, DeviceCapabilityExtension, FragmentationParameters,
    ManufacturerSpecific, NextChannelChange, NextPanIdChange, PanIdConflictReport,
    RouterInformation, SupportedKeyNegotiation, SymmetricPassphrase,
};
use crate::types::tlv::{General, Tag};

/// Encapsulated Global TLV types.
#[expect(variant_size_differences, clippy::large_enum_variant)]
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

    /// Configuration Parameters TLV.
    ConfigurationParameters(ConfigurationParameters),

    /// Device Capability Extension TLV.
    DeviceCapabilityExtension(DeviceCapabilityExtension),
}

impl From<EncapsulatedGlobal> for General {
    fn from(value: EncapsulatedGlobal) -> Self {
        match value {
            EncapsulatedGlobal::ManufacturerSpecific(value) => Self::serialize(value),
            EncapsulatedGlobal::SupportedKeyNegotiationMethods(value) => Self::serialize(value),
            EncapsulatedGlobal::PanIdConflictReport(value) => Self::serialize(value),
            EncapsulatedGlobal::NextPanIdChange(value) => Self::serialize(value),
            EncapsulatedGlobal::NextChannelChange(value) => Self::serialize(value),
            EncapsulatedGlobal::SymmetricPassphrase(value) => Self::serialize(value),
            EncapsulatedGlobal::RouterInformation(value) => Self::serialize(value),
            EncapsulatedGlobal::FragmentationParameters(value) => Self::serialize(value),
            EncapsulatedGlobal::ConfigurationParameters(value) => Self::serialize(value),
            EncapsulatedGlobal::DeviceCapabilityExtension(value) => Self::serialize(value),
        }
    }
}

impl TryFrom<General> for EncapsulatedGlobal {
    type Error = u8;

    fn try_from(general: General) -> Result<Self, Self::Error> {
        let (typ, payload) = general.into_parts();

        match typ {
            ManufacturerSpecific::TAG => {
                ManufacturerSpecific::from_le_stream(payload.to_le_stream())
                    .map(Self::ManufacturerSpecific)
                    .ok_or(typ)
            }
            SupportedKeyNegotiation::TAG => {
                SupportedKeyNegotiation::from_le_stream(payload.to_le_stream())
                    .map(Self::SupportedKeyNegotiationMethods)
                    .ok_or(typ)
            }
            PanIdConflictReport::TAG => PanIdConflictReport::from_le_stream(payload.to_le_stream())
                .map(Self::PanIdConflictReport)
                .ok_or(typ),
            NextPanIdChange::TAG => NextPanIdChange::from_le_stream(payload.to_le_stream())
                .map(Self::NextPanIdChange)
                .ok_or(typ),
            NextChannelChange::TAG => NextChannelChange::from_le_stream(payload.to_le_stream())
                .map(Self::NextChannelChange)
                .ok_or(typ),
            SymmetricPassphrase::TAG => SymmetricPassphrase::from_le_stream(payload.to_le_stream())
                .map(Self::SymmetricPassphrase)
                .ok_or(typ),
            RouterInformation::TAG => RouterInformation::from_le_stream(payload.to_le_stream())
                .map(Self::RouterInformation)
                .ok_or(typ),
            FragmentationParameters::TAG => {
                FragmentationParameters::from_le_stream(payload.to_le_stream())
                    .map(Self::FragmentationParameters)
                    .ok_or(typ)
            }
            ConfigurationParameters::TAG => {
                ConfigurationParameters::from_le_stream(payload.to_le_stream())
                    .map(Self::ConfigurationParameters)
                    .ok_or(typ)
            }
            DeviceCapabilityExtension::TAG => {
                DeviceCapabilityExtension::from_le_stream(payload.to_le_stream())
                    .map(Self::DeviceCapabilityExtension)
                    .ok_or(typ)
            }
            unknown_tag => Err(unknown_tag),
        }
    }
}

impl ToLeStream for EncapsulatedGlobal {
    type Iter = <General as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        General::from(self).to_le_stream()
    }
}
