use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

pub use self::beacon_appendix_encapsulation::BeaconAppendixEncapsulation;
pub use self::configuration_parameters::ConfigurationParameters;
pub use self::device_capability_extension::DeviceCapabilityExtension;
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

mod beacon_appendix_encapsulation;
mod configuration_parameters;
mod device_capability_extension;
mod fragmentation_parameters;
mod joiner_encapsulation;
mod manufacturer_specific;
mod next_channel_change;
mod next_pan_id_change;
mod pan_id_conflict_report;
mod router_information;
mod supported_key_negotiation;
mod symmetric_passphrase;

/// Global TLV tags.
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
    /// BDB Encapsulation TLV.
    BdbEncapsulation,
    /// Configuration Parameters TLV.
    ConfigurationParameters(ConfigurationParameters),
    /// Device Capability Extension TLV.
    DeviceCapabilityExtension(DeviceCapabilityExtension),
}

impl FromLeStreamTagged for Global {
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
            JoinerEncapsulation::TAG => {
                Ok(JoinerEncapsulation::from_le_stream(bytes).map(Self::JoinerEncapsulation))
            }
            BeaconAppendixEncapsulation::TAG => {
                Ok(BeaconAppendixEncapsulation::from_le_stream(bytes)
                    .map(Self::BeaconAppendixEncapsulation))
            }
            // TODO: Define BdbEncapsulation parsing when its structure is known.
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

impl ToLeStream for Global {
    type Iter = iter::GlobalLeStream;

    fn to_le_stream(self) -> Self::Iter {
        iter::GlobalLeStream::from(self)
    }
}

mod iter {
    use le_stream::ToLeStream;

    use super::{
        BeaconAppendixEncapsulation, ConfigurationParameters, DeviceCapabilityExtension,
        FragmentationParameters, JoinerEncapsulation, ManufacturerSpecific, NextChannelChange,
        NextPanIdChange, PanIdConflictReport, RouterInformation, SupportedKeyNegotiation,
        SymmetricPassphrase,
    };
    use crate::types::tlv::Global;

    pub enum GlobalLeStream {
        ManufacturerSpecific(<ManufacturerSpecific as ToLeStream>::Iter),
        SupportedKeyNegotiationMethods(<SupportedKeyNegotiation as ToLeStream>::Iter),
        PanIdConflictReport(<PanIdConflictReport as ToLeStream>::Iter),
        NextPanIdChange(<NextPanIdChange as ToLeStream>::Iter),
        NextChannelChange(<NextChannelChange as ToLeStream>::Iter),
        SymmetricPassphrase(<SymmetricPassphrase as ToLeStream>::Iter),
        RouterInformation(<RouterInformation as ToLeStream>::Iter),
        FragmentationParameters(<FragmentationParameters as ToLeStream>::Iter),
        JoinerEncapsulation(<JoinerEncapsulation as ToLeStream>::Iter),
        BeaconAppendixEncapsulation(<BeaconAppendixEncapsulation as ToLeStream>::Iter),
        BdbEncapsulation(<() as ToLeStream>::Iter),
        ConfigurationParameters(<ConfigurationParameters as ToLeStream>::Iter),
        DeviceCapabilityExtension(<DeviceCapabilityExtension as ToLeStream>::Iter),
    }

    impl Iterator for GlobalLeStream {
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
                Self::JoinerEncapsulation(iter) => iter.next(),
                Self::BeaconAppendixEncapsulation(iter) => iter.next(),
                Self::BdbEncapsulation(iter) => iter.next(),
                Self::ConfigurationParameters(iter) => iter.next(),
                Self::DeviceCapabilityExtension(iter) => iter.next(),
            }
        }
    }

    impl From<Global> for GlobalLeStream {
        fn from(global: Global) -> Self {
            match global {
                Global::ManufacturerSpecific(value) => {
                    Self::ManufacturerSpecific(value.to_le_stream())
                }
                Global::SupportedKeyNegotiationMethods(value) => {
                    Self::SupportedKeyNegotiationMethods(value.to_le_stream())
                }
                Global::PanIdConflictReport(value) => {
                    Self::PanIdConflictReport(value.to_le_stream())
                }
                Global::NextPanIdChange(value) => Self::NextPanIdChange(value.to_le_stream()),
                Global::NextChannelChange(value) => Self::NextChannelChange(value.to_le_stream()),
                Global::SymmetricPassphrase(value) => {
                    Self::SymmetricPassphrase(value.to_le_stream())
                }
                Global::RouterInformation(value) => Self::RouterInformation(value.to_le_stream()),
                Global::FragmentationParameters(value) => {
                    Self::FragmentationParameters(value.to_le_stream())
                }
                Global::JoinerEncapsulation(value) => {
                    Self::JoinerEncapsulation(value.to_le_stream())
                }
                Global::BeaconAppendixEncapsulation(value) => {
                    Self::BeaconAppendixEncapsulation(value.to_le_stream())
                }
                Global::BdbEncapsulation => Self::BdbEncapsulation(().to_le_stream()),
                Global::ConfigurationParameters(value) => {
                    Self::ConfigurationParameters(value.to_le_stream())
                }
                Global::DeviceCapabilityExtension(value) => {
                    Self::DeviceCapabilityExtension(value.to_le_stream())
                }
            }
        }
    }
}
