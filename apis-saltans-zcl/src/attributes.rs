use apis_saltans_core::Cluster;
use apis_saltans_core::types::Type;

pub use self::errors::{InvalidType, ParseAttributeError};
use crate::clusters::{general, ias, lighting, measurement_and_sensing};
use crate::global::write_attributes::Record;

mod errors;

type BasicAttributes = general::basic::attributes::Reportable;
type PowerConfigurationAttributes = general::power_configuration::attributes::Reportable;
type DeviceTemperatureConfigurationAttributes =
    general::device_temperature_configuration::attributes::Reportable;
type IdentifyAttributes = general::identify::attributes::Reportable;
type GroupsAttributes = general::groups::attributes::Reportable;
type ScenesAttributes = general::scenes::attributes::Reportable;
type OnOffAttributes = general::on_off::attributes::Reportable;
type LevelAttributes = general::level::attributes::Reportable;
type AlarmsAttributes = general::alarms::attributes::Reportable;
type TimeAttributes = general::time::attributes::Reportable;
type IlluminanceMeasurementAttributes =
    measurement_and_sensing::illuminance_measurement::attributes::Reportable;
type IlluminanceLevelSensingAttributes =
    measurement_and_sensing::illuminance_level_sensing::attributes::Reportable;
type OccupancySensingAttributes =
    measurement_and_sensing::occupancy_sensing::attributes::Reportable;
type BallastConfigurationAttributes = lighting::ballast_configuration::attributes::Reportable;
type ColorControlAttributes = lighting::color_control::attributes::Reportable;
type IasZoneAttributes = ias::zone::attributes::Reportable;

/// A trait to allow the reading of attributes by their respective IDs in a type-safe manner.
pub trait Readable: Cluster + TryFrom<u16, Error = u16> + Into<u16> {
    /// The manufacturer code of the attribute, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// The type of attribute, usually an enum, which is returned from the readable.
    type Attribute: TryFrom<(Self, Type), Error = InvalidType<Self>>;
}

/// A trait to allow the writing of attribute values in a type-safe manner.
pub trait Writable: Cluster + Into<Record> {
    /// The manufacturer code of the attribute, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// The ID of the attribute.
    fn id(&self) -> u16;
}

/// Reportable attributes of all implemented ZCL clusters.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Reportable {
    /// Reportable attributes of the Basic cluster.
    Basic(general::basic::attributes::Reportable),
    /// Reportable attributes of the Power Configuration cluster.
    PowerConfiguration(general::power_configuration::attributes::Reportable),
    /// Reportable attributes of the Device Temperature Configuration cluster.
    DeviceTemperatureConfiguration(
        general::device_temperature_configuration::attributes::Reportable,
    ),
    /// Reportable attributes of the Identify cluster.
    Identify(general::identify::attributes::Reportable),
    /// Reportable attributes of the Groups cluster.
    Groups(general::groups::attributes::Reportable),
    /// Reportable attributes of the Scenes cluster.
    Scenes(general::scenes::attributes::Reportable),
    /// Reportable attributes of the On/Off cluster.
    OnOff(general::on_off::attributes::Reportable),
    /// Reportable attributes of the Level Control cluster.
    Level(general::level::attributes::Reportable),
    /// Reportable attributes of the Alarms cluster.
    Alarms(general::alarms::attributes::Reportable),
    /// Reportable attributes of the Time cluster.
    Time(general::time::attributes::Reportable),
    /// Reportable attributes of the Illuminance Measurement cluster.
    IlluminanceMeasurement(
        measurement_and_sensing::illuminance_measurement::attributes::Reportable,
    ),
    /// Reportable attributes of the Illuminance Level Sensing cluster.
    IlluminanceLevelSensing(
        measurement_and_sensing::illuminance_level_sensing::attributes::Reportable,
    ),
    /// Reportable attributes of the Occupancy Sensing cluster.
    OccupancySensing(measurement_and_sensing::occupancy_sensing::attributes::Reportable),
    /// Reportable attributes of the Ballast Configuration cluster.
    BallastConfiguration(lighting::ballast_configuration::attributes::Reportable),
    /// Reportable attributes of the Color Control cluster.
    ColorControl(lighting::color_control::attributes::Reportable),
    /// Reportable attributes of the IAS Zone cluster.
    IasZone(ias::zone::attributes::Reportable),
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
            <BasicAttributes as Cluster>::ID => parse_cluster!(BasicAttributes, Basic),
            <PowerConfigurationAttributes as Cluster>::ID => {
                parse_cluster!(PowerConfigurationAttributes, PowerConfiguration)
            }
            <DeviceTemperatureConfigurationAttributes as Cluster>::ID => parse_cluster!(
                DeviceTemperatureConfigurationAttributes,
                DeviceTemperatureConfiguration
            ),
            <IdentifyAttributes as Cluster>::ID => parse_cluster!(IdentifyAttributes, Identify),
            <GroupsAttributes as Cluster>::ID => parse_cluster!(GroupsAttributes, Groups),
            <ScenesAttributes as Cluster>::ID => parse_cluster!(ScenesAttributes, Scenes),
            <OnOffAttributes as Cluster>::ID => parse_cluster!(OnOffAttributes, OnOff),
            <LevelAttributes as Cluster>::ID => parse_cluster!(LevelAttributes, Level),
            <AlarmsAttributes as Cluster>::ID => parse_cluster!(AlarmsAttributes, Alarms),
            <TimeAttributes as Cluster>::ID => parse_cluster!(TimeAttributes, Time),
            <IlluminanceMeasurementAttributes as Cluster>::ID => {
                parse_cluster!(IlluminanceMeasurementAttributes, IlluminanceMeasurement)
            }
            <IlluminanceLevelSensingAttributes as Cluster>::ID => {
                parse_cluster!(IlluminanceLevelSensingAttributes, IlluminanceLevelSensing)
            }
            <OccupancySensingAttributes as Cluster>::ID => {
                parse_cluster!(OccupancySensingAttributes, OccupancySensing)
            }
            <BallastConfigurationAttributes as Cluster>::ID => {
                parse_cluster!(BallastConfigurationAttributes, BallastConfiguration)
            }
            <ColorControlAttributes as Cluster>::ID => {
                parse_cluster!(ColorControlAttributes, ColorControl)
            }
            <IasZoneAttributes as Cluster>::ID => parse_cluster!(IasZoneAttributes, IasZone),
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
    use apis_saltans_core::ClusterId;
    use apis_saltans_core::types::{Bool, Type, Uint8};

    use super::{ParseAttributeError, Reportable};
    use crate::clusters::general;

    #[test]
    fn parses_reportable_attribute() {
        let attribute = Reportable::parse(
            ClusterId::Level.as_u16(),
            0x0000,
            Type::Uint8(Uint8::new(42)),
        )
        .expect("reportable attribute should parse");

        assert_eq!(
            attribute,
            Reportable::Level(general::level::attributes::Reportable::CurrentLevel(
                Uint8::new(42)
            ))
        );
    }

    #[test]
    fn rejects_non_reportable_attribute_id() {
        let error = Reportable::parse(
            ClusterId::Level.as_u16(),
            0x0001,
            Type::Uint8(Uint8::new(42)),
        )
        .expect_err("non-reportable attribute should fail");

        assert_eq!(error, ParseAttributeError::InvalidId(0x0001));
    }

    #[test]
    fn rejects_invalid_reportable_attribute_type() {
        let error = Reportable::parse(ClusterId::Level.as_u16(), 0x0000, Type::Boolean(Bool::TRUE))
            .expect_err("wrong attribute type should fail");

        assert!(matches!(error, ParseAttributeError::InvalidType(_)));
    }
}
