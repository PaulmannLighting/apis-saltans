use le_stream::{FromLeStream, ToLeStream};

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
use super::{General, Payload, Tag};

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

impl From<Global> for General {
    fn from(global: Global) -> Self {
        match global {
            Global::ManufacturerSpecific(value) => Self::serialize(value),
            Global::SupportedKeyNegotiationMethods(value) => Self::serialize(value),
            Global::PanIdConflictReport(value) => Self::serialize(value),
            Global::NextPanIdChange(value) => Self::serialize(value),
            Global::NextChannelChange(value) => Self::serialize(value),
            Global::SymmetricPassphrase(value) => Self::serialize(value),
            Global::RouterInformation(value) => Self::serialize(value),
            Global::FragmentationParameters(value) => Self::serialize(value),
            Global::JoinerEncapsulation(value) => Self::from(value),
            Global::BeaconAppendixEncapsulation(value) => Self::from(value),
            Global::BdbEncapsulation => Self::new(74, Payload::new()),
            Global::ConfigurationParameters(value) => Self::serialize(value),
            Global::DeviceCapabilityExtension(value) => Self::serialize(value),
        }
    }
}

impl TryFrom<General> for Global {
    type Error = u8;

    fn try_from(general: General) -> Result<Self, Self::Error> {
        let (typ, payload) = general.into_parts();

        match typ {
            ManufacturerSpecific::TAG => ManufacturerSpecific::from_le_stream(payload.into_iter())
                .map(Self::ManufacturerSpecific)
                .ok_or(typ),
            SupportedKeyNegotiation::TAG => {
                SupportedKeyNegotiation::from_le_stream(payload.into_iter())
                    .map(Self::SupportedKeyNegotiationMethods)
                    .ok_or(typ)
            }
            PanIdConflictReport::TAG => PanIdConflictReport::from_le_stream(payload.into_iter())
                .map(Self::PanIdConflictReport)
                .ok_or(typ),
            NextPanIdChange::TAG => NextPanIdChange::from_le_stream(payload.into_iter())
                .map(Self::NextPanIdChange)
                .ok_or(typ),
            NextChannelChange::TAG => NextChannelChange::from_le_stream(payload.into_iter())
                .map(Self::NextChannelChange)
                .ok_or(typ),
            SymmetricPassphrase::TAG => SymmetricPassphrase::from_le_stream(payload.into_iter())
                .map(Self::SymmetricPassphrase)
                .ok_or(typ),
            RouterInformation::TAG => RouterInformation::from_le_stream(payload.into_iter())
                .map(Self::RouterInformation)
                .ok_or(typ),
            FragmentationParameters::TAG => {
                FragmentationParameters::from_le_stream(payload.into_iter())
                    .map(Self::FragmentationParameters)
                    .ok_or(typ)
            }
            JoinerEncapsulation::TAG => Ok(Self::JoinerEncapsulation(JoinerEncapsulation::from(
                payload,
            ))),
            BeaconAppendixEncapsulation::TAG => Ok(Self::BeaconAppendixEncapsulation(
                BeaconAppendixEncapsulation::from(payload),
            )),
            // TODO: Define BdbEncapsulation parsing when its structure is known.
            ConfigurationParameters::TAG => {
                ConfigurationParameters::from_le_stream(payload.into_iter())
                    .map(Self::ConfigurationParameters)
                    .ok_or(typ)
            }
            DeviceCapabilityExtension::TAG => {
                DeviceCapabilityExtension::from_le_stream(payload.into_iter())
                    .map(Self::DeviceCapabilityExtension)
                    .ok_or(typ)
            }
            unknown_tag => Err(unknown_tag),
        }
    }
}

impl ToLeStream for Global {
    type Iter = <General as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        General::from(self).to_le_stream()
    }
}
