use zb_core::types::Type;
use zb_core::{ClusterSpecific, Profiled};

pub use self::errors::{InvalidType, ParseAttributeError};
use crate::alarms::Reportable as AlarmsAttributes;
use crate::ballast_configuration::Reportable as BallastConfigurationAttributes;
use crate::basic::Reportable as BasicAttributes;
use crate::color_control::Reportable as ColorControlAttributes;
use crate::device_temperature_configuration::Reportable as DeviceTemperatureConfigurationAttributes;
use crate::global::write_attributes::Record;
use crate::groups::Reportable as GroupsAttributes;
use crate::ias::zone::Reportable as IasZoneAttributes;
use crate::identify::Reportable as IdentifyAttributes;
use crate::illuminance_level_sensing::Reportable as IlluminanceLevelSensingAttributes;
use crate::illuminance_measurement::Reportable as IlluminanceMeasurementAttributes;
use crate::level::Reportable as LevelAttributes;
use crate::occupancy_sensing::Reportable as OccupancySensingAttributes;
use crate::on_off::Reportable as OnOffAttributes;
use crate::power_configuration::Reportable as PowerConfigurationAttributes;
use crate::scenes::Reportable as ScenesAttributes;
use crate::time::Reportable as TimeAttributes;

mod errors;

/// A trait to allow the reading of attributes by their respective IDs in a type-safe manner.
pub trait Readable: ClusterSpecific + Profiled + TryFrom<u16, Error = u16> + Into<u16> {
    /// The manufacturer code, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// The type of attribute, usually an enum, which is returned from the readable.
    type Attribute: TryFrom<(Self, Type), Error = InvalidType<Self>>;
}

/// A trait to allow the writing of attribute values in a type-safe manner.
pub trait Writable: ClusterSpecific + Profiled + Into<Record> {
    /// The manufacturer code, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// The ID of the attribute.
    fn id(&self) -> u16;
}

/// Reportable attributes of all implemented ZCL clusters.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Reportable {
    /// Reportable attributes of the Basic cluster.
    Basic(BasicAttributes),
    /// Reportable attributes of the Power Configuration cluster.
    PowerConfiguration(PowerConfigurationAttributes),
    /// Reportable attributes of the Device Temperature Configuration cluster.
    DeviceTemperatureConfiguration(DeviceTemperatureConfigurationAttributes),
    /// Reportable attributes of the Identify cluster.
    Identify(IdentifyAttributes),
    /// Reportable attributes of the Groups cluster.
    Groups(GroupsAttributes),
    /// Reportable attributes of the Scenes cluster.
    Scenes(ScenesAttributes),
    /// Reportable attributes of the On/Off cluster.
    OnOff(OnOffAttributes),
    /// Reportable attributes of the Level Control cluster.
    Level(LevelAttributes),
    /// Reportable attributes of the Alarms cluster.
    Alarms(AlarmsAttributes),
    /// Reportable attributes of the Time cluster.
    Time(TimeAttributes),
    /// Reportable attributes of the Illuminance Measurement cluster.
    IlluminanceMeasurement(IlluminanceMeasurementAttributes),
    /// Reportable attributes of the Illuminance Level Sensing cluster.
    IlluminanceLevelSensing(IlluminanceLevelSensingAttributes),
    /// Reportable attributes of the Occupancy Sensing cluster.
    OccupancySensing(OccupancySensingAttributes),
    /// Reportable attributes of the Ballast Configuration cluster.
    BallastConfiguration(BallastConfigurationAttributes),
    /// Reportable attributes of the Color Control cluster.
    ColorControl(ColorControlAttributes),
    /// Reportable attributes of the IAS Zone cluster.
    IasZone(IasZoneAttributes),
}

impl Reportable {
    /// Parse a reportable attribute from a cluster ID, attribute ID, and ZCL type.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseAttributeError`] if the cluster or attribute is unsupported, or if the
    /// provided type does not match the reportable attribute.
    pub fn parse(
        cluster_id: u16,
        attribute_id: u16,
        typ: Type,
    ) -> Result<Self, ParseAttributeError<u16>> {
        macro_rules! parse_cluster {
            ($attributes:ty, $variant:ident) => {
                Self::parse_cluster::<$attributes, _>(attribute_id, typ, Self::$variant)
            };
        }

        match cluster_id {
            <BasicAttributes as ClusterSpecific>::ID => parse_cluster!(BasicAttributes, Basic),
            <PowerConfigurationAttributes as ClusterSpecific>::ID => {
                parse_cluster!(PowerConfigurationAttributes, PowerConfiguration)
            }
            <DeviceTemperatureConfigurationAttributes as ClusterSpecific>::ID => parse_cluster!(
                DeviceTemperatureConfigurationAttributes,
                DeviceTemperatureConfiguration
            ),
            <IdentifyAttributes as ClusterSpecific>::ID => {
                parse_cluster!(IdentifyAttributes, Identify)
            }
            <GroupsAttributes as ClusterSpecific>::ID => parse_cluster!(GroupsAttributes, Groups),
            <ScenesAttributes as ClusterSpecific>::ID => parse_cluster!(ScenesAttributes, Scenes),
            <OnOffAttributes as ClusterSpecific>::ID => parse_cluster!(OnOffAttributes, OnOff),
            <LevelAttributes as ClusterSpecific>::ID => parse_cluster!(LevelAttributes, Level),
            <AlarmsAttributes as ClusterSpecific>::ID => parse_cluster!(AlarmsAttributes, Alarms),
            <TimeAttributes as ClusterSpecific>::ID => parse_cluster!(TimeAttributes, Time),
            <IlluminanceMeasurementAttributes as ClusterSpecific>::ID => {
                parse_cluster!(IlluminanceMeasurementAttributes, IlluminanceMeasurement)
            }
            <IlluminanceLevelSensingAttributes as ClusterSpecific>::ID => {
                parse_cluster!(IlluminanceLevelSensingAttributes, IlluminanceLevelSensing)
            }
            <OccupancySensingAttributes as ClusterSpecific>::ID => {
                parse_cluster!(OccupancySensingAttributes, OccupancySensing)
            }
            <BallastConfigurationAttributes as ClusterSpecific>::ID => {
                parse_cluster!(BallastConfigurationAttributes, BallastConfiguration)
            }
            <ColorControlAttributes as ClusterSpecific>::ID => {
                parse_cluster!(ColorControlAttributes, ColorControl)
            }
            <IasZoneAttributes as ClusterSpecific>::ID => {
                parse_cluster!(IasZoneAttributes, IasZone)
            }
            _ => Err(ParseAttributeError::InvalidId(attribute_id)),
        }
    }

    fn parse_cluster<T, F>(
        attribute_id: u16,
        typ: Type,
        convert: F,
    ) -> Result<Self, ParseAttributeError<u16>>
    where
        T: TryFrom<(u16, Type), Error = ParseAttributeError<u16>>,
        F: FnOnce(T) -> Self,
    {
        T::try_from((attribute_id, typ)).map(convert)
    }
}

#[cfg(test)]
mod tests {
    use zb_core::Cluster;
    use zb_core::types::{Bool, Type, Uint8};

    use super::{ParseAttributeError, Reportable};
    use crate::clusters::general;

    #[test]
    fn parses_reportable_attribute() {
        let attribute =
            Reportable::parse(Cluster::Level.as_u16(), 0x0000, Type::Uint8(Uint8::new(42)))
                .expect("reportable attribute should parse");

        assert_eq!(
            attribute,
            Reportable::Level(general::level::Reportable::CurrentLevel(Uint8::new(42)))
        );
    }

    #[test]
    fn rejects_non_reportable_attribute_id() {
        let error = Reportable::parse(Cluster::Level.as_u16(), 0x0001, Type::Uint8(Uint8::new(42)))
            .expect_err("non-reportable attribute should fail");

        assert_eq!(error, ParseAttributeError::InvalidId(0x0001));
    }

    #[test]
    fn rejects_invalid_reportable_attribute_type() {
        let error = Reportable::parse(Cluster::Level.as_u16(), 0x0000, Type::Boolean(Bool::TRUE))
            .expect_err("wrong attribute type should fail");

        assert!(matches!(error, ParseAttributeError::InvalidType(_)));
    }
}
