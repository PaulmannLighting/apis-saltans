use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::Tag;
use crate::types::tlv::global::{
    ConfigurationParameters, DeviceCapabilityExtension, FragmentationParameters,
    ManufacturerSpecific, NextChannelChange, NextPanIdChange, PanIdConflictReport,
    RouterInformation, SupportedKeyNegotiation, SymmetricPassphrase,
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
    /// Configuration Parameters TLV.
    ConfigurationParameters(ConfigurationParameters),
    /// Device Capability Extension TLV.
    DeviceCapabilityExtension(DeviceCapabilityExtension),
}

impl FromLeStreamTagged for EncapsulatedGlobal {
    type Tag = u8;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            ManufacturerSpecific::TAG => {
                Ok(ManufacturerSpecific::from_le_stream(bytes).map(Self::ManufacturerSpecific))
            }
            SupportedKeyNegotiation::TAG => Ok(SupportedKeyNegotiation::from_le_stream(bytes)
                .map(Self::SupportedKeyNegotiationMethods)),
            PanIdConflictReport::TAG => {
                Ok(PanIdConflictReport::from_le_stream(bytes).map(Self::PanIdConflictReport))
            }
            NextPanIdChange::TAG => {
                Ok(NextPanIdChange::from_le_stream(bytes).map(Self::NextPanIdChange))
            }
            NextChannelChange::TAG => {
                Ok(NextChannelChange::from_le_stream(bytes).map(Self::NextChannelChange))
            }
            SymmetricPassphrase::TAG => {
                Ok(SymmetricPassphrase::from_le_stream(bytes).map(Self::SymmetricPassphrase))
            }
            RouterInformation::TAG => {
                Ok(RouterInformation::from_le_stream(bytes).map(Self::RouterInformation))
            }
            FragmentationParameters::TAG => {
                Ok(FragmentationParameters::from_le_stream(bytes)
                    .map(Self::FragmentationParameters))
            }
            ConfigurationParameters::TAG => {
                Ok(ConfigurationParameters::from_le_stream(bytes)
                    .map(Self::ConfigurationParameters))
            }
            DeviceCapabilityExtension::TAG => Ok(DeviceCapabilityExtension::from_le_stream(bytes)
                .map(Self::DeviceCapabilityExtension)),
            unknown_tag => Err(unknown_tag),
        }
    }
}

impl ToLeStream for EncapsulatedGlobal {
    type Iter = iter::EncapsulatedGlobalIter;

    fn to_le_stream(self) -> Self::Iter {
        iter::EncapsulatedGlobalIter::from(self)
    }
}

mod iter {
    use le_stream::ToLeStream;

    use crate::types::tlv::global::{ConfigurationParameters, FragmentationParameters};
    use crate::types::tlv::{
        DeviceCapabilityExtension, EncapsulatedGlobal, ManufacturerSpecific, NextChannelChange,
        NextPanIdChange, PanIdConflictReport, RouterInformation, SupportedKeyNegotiation,
        SymmetricPassphrase,
    };

    pub enum EncapsulatedGlobalIter {
        ManufacturerSpecific(<ManufacturerSpecific as ToLeStream>::Iter),
        SupportedKeyNegotiationMethods(<SupportedKeyNegotiation as ToLeStream>::Iter),
        PanIdConflictReport(<PanIdConflictReport as ToLeStream>::Iter),
        NextPanIdChange(<NextPanIdChange as ToLeStream>::Iter),
        NextChannelChange(<NextChannelChange as ToLeStream>::Iter),
        SymmetricPassphrase(<SymmetricPassphrase as ToLeStream>::Iter),
        RouterInformation(<RouterInformation as ToLeStream>::Iter),
        FragmentationParameters(<FragmentationParameters as ToLeStream>::Iter),
        ConfigurationParameters(<ConfigurationParameters as ToLeStream>::Iter),
        DeviceCapabilityExtension(<DeviceCapabilityExtension as ToLeStream>::Iter),
    }

    impl Iterator for EncapsulatedGlobalIter {
        type Item = u8;

        #[expect(clippy::match_same_arms)]
        fn next(&mut self) -> Option<Self::Item> {
            match self {
                Self::ManufacturerSpecific(iter) => iter.next(),
                Self::SupportedKeyNegotiationMethods(iter) => iter.next(),
                Self::PanIdConflictReport(iter) => iter.next(),
                Self::NextPanIdChange(iter) => iter.next(),
                Self::NextChannelChange(iter) => iter.next(),
                Self::SymmetricPassphrase(iter) => iter.next(),
                Self::RouterInformation(iter) => iter.next(),
                Self::FragmentationParameters(iter) => iter.next(),
                Self::ConfigurationParameters(iter) => iter.next(),
                Self::DeviceCapabilityExtension(iter) => iter.next(),
            }
        }
    }

    impl From<EncapsulatedGlobal> for EncapsulatedGlobalIter {
        fn from(encapsulated_global: EncapsulatedGlobal) -> Self {
            match encapsulated_global {
                EncapsulatedGlobal::ManufacturerSpecific(value) => {
                    Self::ManufacturerSpecific(value.to_le_stream())
                }
                EncapsulatedGlobal::SupportedKeyNegotiationMethods(value) => {
                    Self::SupportedKeyNegotiationMethods(value.to_le_stream())
                }
                EncapsulatedGlobal::PanIdConflictReport(value) => {
                    Self::PanIdConflictReport(value.to_le_stream())
                }
                EncapsulatedGlobal::NextPanIdChange(value) => {
                    Self::NextPanIdChange(value.to_le_stream())
                }
                EncapsulatedGlobal::NextChannelChange(value) => {
                    Self::NextChannelChange(value.to_le_stream())
                }
                EncapsulatedGlobal::SymmetricPassphrase(value) => {
                    Self::SymmetricPassphrase(value.to_le_stream())
                }
                EncapsulatedGlobal::RouterInformation(value) => {
                    Self::RouterInformation(value.to_le_stream())
                }
                EncapsulatedGlobal::FragmentationParameters(value) => {
                    Self::FragmentationParameters(value.to_le_stream())
                }
                EncapsulatedGlobal::ConfigurationParameters(value) => {
                    Self::ConfigurationParameters(value.to_le_stream())
                }
                EncapsulatedGlobal::DeviceCapabilityExtension(value) => {
                    Self::DeviceCapabilityExtension(value.to_le_stream())
                }
            }
        }
    }
}
