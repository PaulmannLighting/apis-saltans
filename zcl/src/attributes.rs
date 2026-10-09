use zb_core::types::Type;
use zb_core::{ClusterSpecific, Profiled};

pub use self::analog::Analog;
pub use self::discrete::Discrete;
pub use self::errors::{InvalidType, ParseAttributeError};
use crate::alarms::Reportable as AlarmsAttributes;
use crate::ballast_configuration::Reportable as BallastConfigurationAttributes;
use crate::basic::Reportable as BasicAttributes;
use crate::color_control::Reportable as ColorControlAttributes;
use crate::device_temperature_configuration::Reportable as DeviceTemperatureConfigurationAttributes;
use crate::global::configure_reporting;
use crate::global::write_attributes::Record;
use crate::groups::Reportable as GroupsAttributes;
use crate::ias::zone::Reportable as IasZoneAttributes;
use crate::identify::Reportable as IdentifyAttributes;
use crate::illuminance_level_sensing::Reportable as IlluminanceLevelSensingAttributes;
use crate::illuminance_measurement::Reportable as IlluminanceMeasurementAttributes;
use crate::level::Reportable as LevelAttributes;
use crate::occupancy_sensing::Reportable as OccupancySensingAttributes;
use crate::on_off::Reportable as OnOffAttributes;
use crate::ota_upgrade::Reportable as OtaUpgradeAttributes;
use crate::power_configuration::Reportable as PowerConfigurationAttributes;
use crate::scenes::Reportable as ScenesAttributes;
use crate::time::Reportable as TimeAttributes;

mod analog;
mod discrete;
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

/// A trait for reporting configurations and their ZCL wire types.
///
/// Generated implementations include attributes without `P`: reporting may be a manufacturer
/// option (ZCL §2.5.7.3). Availability here does not guarantee device support or override
/// attribute-specific restrictions. Discover Attributes Extended describes actual device access.
pub trait Reportable:
    ClusterSpecific + Profiled + Into<configure_reporting::send::AttributeReportingConfiguration>
{
    /// The manufacturer code, if any.
    const MANUFACTURER_CODE: Option<u16> = None;

    /// Return the attribute ID.
    fn attribute_id(&self) -> u16;

    /// Return the ZCL data type ID.
    fn type_id(&self) -> u8;
}

/// Reportable attributes of all implemented ZCL clusters.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AttributeReport {
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

    /// Reportable attributes of the OTA Upgrade cluster.
    OtaUpgrade(OtaUpgradeAttributes),

    /// Reportable attributes of the IAS Zone cluster.
    IasZone(IasZoneAttributes),
}

impl AttributeReport {
    /// Parse a reportable attribute from a cluster ID, attribute ID, and ZCL type.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseAttributeError`] if the cluster or attribute is unsupported, or if the
    /// provided type does not match the attribute. Attributes without `P` are accepted too.
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
            <OtaUpgradeAttributes as ClusterSpecific>::ID => {
                parse_cluster!(OtaUpgradeAttributes, OtaUpgrade)
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
    use zb_core::types::{Bool, Type, Uint8, Uint16};

    use super::{AttributeReport, ParseAttributeError};
    use crate::clusters::general;

    #[test]
    fn parses_reportable_attribute() {
        let attribute =
            AttributeReport::parse(Cluster::Level.as_u16(), 0x0000, Type::Uint8(Uint8::new(42)))
                .expect("reportable attribute should parse");

        assert_eq!(
            attribute,
            AttributeReport::Level(general::level::Reportable::CurrentLevel(Uint8::new(42)))
        );
    }

    #[test]
    fn parses_optionally_reported_attribute() {
        const REMAINING_TIME_ID: u16 = 0x0001;
        const REMAINING_TIME: Uint16 = Uint16::new(42);

        let attribute = AttributeReport::parse(
            Cluster::Level.as_u16(),
            REMAINING_TIME_ID,
            Type::Uint16(REMAINING_TIME),
        )
        .expect("reporting without P is a manufacturer option");

        assert_eq!(
            attribute,
            AttributeReport::Level(general::level::Reportable::RemainingTime(REMAINING_TIME))
        );
        assert!(matches!(
            AttributeReport::parse(
                Cluster::Level.as_u16(),
                REMAINING_TIME_ID,
                Type::Boolean(Bool::TRUE)
            ),
            Err(ParseAttributeError::InvalidType(_))
        ));
    }

    #[test]
    fn parses_global_report_and_rejects_unknown_attribute() {
        const CLUSTER_REVISION_ID: u16 = 0xfffd;
        const UNKNOWN_ID: u16 = 0xeeee;
        const REVISION: Uint16 = Uint16::new(8);

        assert_eq!(
            AttributeReport::parse(
                Cluster::Basic.as_u16(),
                CLUSTER_REVISION_ID,
                Type::Uint16(REVISION)
            ),
            Ok(AttributeReport::Basic(
                general::basic::Reportable::ClusterRevision(REVISION)
            ))
        );
        assert_eq!(
            AttributeReport::parse(Cluster::Basic.as_u16(), UNKNOWN_ID, Type::Uint16(REVISION)),
            Err(ParseAttributeError::InvalidId(UNKNOWN_ID))
        );
    }

    #[test]
    fn rejects_invalid_reportable_attribute_type() {
        let error =
            AttributeReport::parse(Cluster::Level.as_u16(), 0x0000, Type::Boolean(Bool::TRUE))
                .expect_err("wrong attribute type should fail");

        assert!(matches!(error, ParseAttributeError::InvalidType(_)));
    }
}
