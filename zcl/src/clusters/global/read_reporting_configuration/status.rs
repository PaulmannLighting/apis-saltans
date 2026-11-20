use core::iter::once;
use core::ops::RangeInclusive;

use heapless::Vec;
use le_stream::{FromLeStream, ToLeStream};
use repr_discriminant::ReprDiscriminant;
use zb_core::types::Type;

#[cfg(test)]
mod tests;

const SUCCESS: u8 = 0x00;
const REPORTED_ATTRIBUTE: u8 = 0x00;
const RECEIVED_ATTRIBUTE: u8 = 0x01;
const REPORTED_HEADER_BYTES: usize = 9;
const MAX_SUPPORTED_CHANGE_BYTES: usize = 8;

// ZCL data type classes. Discrete reporting records carry only the datatype ID,
// including for types whose values are not represented by zb_core::types::Type.
const DISCRETE_TYPES: [RangeInclusive<u8>; 6] = [
    0x08..=0x10, // General data and Boolean
    0x18..=0x1f, // Bitmaps
    0x30..=0x31, // Enumerations
    0x41..=0x44, // Strings
    0xe8..=0xea, // Identifiers
    0xf0..=0xf1, // IEEE address and security key
];
const SUPPORTED_ANALOG_TYPES: [RangeInclusive<u8>; 2] = [
    0x20..=0x2f, // Unsigned and signed integers
    0xe0..=0xe2, // Time of day, date and UTC time
];

type RecordBytes = Vec<u8, { REPORTED_HEADER_BYTES + MAX_SUPPORTED_CHANGE_BYTES }>;

/// Parameters of an attribute reported by the target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReportedConfiguration {
    /// ZCL wire type of the attribute.
    pub attribute_data_type: u8,
    /// Minimum time between reports, in seconds.
    pub minimum_reporting_interval: u16,
    /// Maximum time between reports, in seconds.
    pub maximum_reporting_interval: u16,
    /// Change threshold for analog attributes. Discrete attributes carry no threshold.
    pub reportable_change: Option<Type>,
}

/// A reporting configuration cannot be represented by the response codec.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum InvalidReportingRecord {
    /// The attribute datatype is unknown or has an unsupported reportable-change encoding.
    #[error("unsupported reporting attribute datatype {0:#04x}")]
    UnsupportedDatatype(u8),

    /// The threshold is missing, unexpected, or a different type from the attribute.
    #[error("reportable change does not match attribute datatype {0:#04x}")]
    InvalidReportableChange(u8),

    /// A failed record requires a non-success status and one of the two reporting directions.
    #[error(
        "invalid failed reporting record with status {status:#04x} and direction {direction:#04x}"
    )]
    InvalidFailure {
        /// Raw ZCL status.
        status: u8,
        /// Raw reporting direction.
        direction: u8,
    },
}

/// Attribute reporting configuration record of a Read Reporting Configuration response.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Status(StatusFields);

/// The flat Serde representation retained for existing records.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename = "Status")
)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct StatusFields {
    code: u8,
    direction: u8,
    attribute_id: u16,
    attribute_data_type: Option<u8>,
    minimum_reporting_interval: Option<u16>,
    maximum_reporting_interval: Option<u16>,
    reportable_change: Option<Type>,
    timeout_period: Option<u16>,
}

#[derive(Clone, Copy)]
enum ChangeEncoding {
    Discrete,

    Analog,
}

impl ChangeEncoding {
    fn classify(datatype: u8) -> Result<Self, InvalidReportingRecord> {
        if DISCRETE_TYPES.iter().any(|range| range.contains(&datatype)) {
            Ok(Self::Discrete)
        } else if SUPPORTED_ANALOG_TYPES
            .iter()
            .any(|range| range.contains(&datatype))
        {
            Ok(Self::Analog)
        } else {
            Err(InvalidReportingRecord::UnsupportedDatatype(datatype))
        }
    }

    fn validate(datatype: u8, change: Option<&Type>) -> Result<(), InvalidReportingRecord> {
        match (Self::classify(datatype)?, change) {
            (Self::Discrete, None) => Ok(()),
            (Self::Analog, Some(change)) if change.repr_discriminant() == datatype => Ok(()),
            _ => Err(InvalidReportingRecord::InvalidReportableChange(datatype)),
        }
    }
}

impl Status {
    /// Create a successful record for an attribute reported by the target.
    ///
    /// # Panics
    /// Panics if the datatype or reportable change is invalid. Use [`Self::try_reported`] for
    /// configurations constructed from external inputs.
    #[must_use]
    #[track_caller]
    pub fn reported(attribute_id: u16, configuration: ReportedConfiguration) -> Self {
        Self::try_reported(attribute_id, configuration).expect("valid reported configuration")
    }

    /// Validate and construct a successful sender record.
    ///
    /// # Errors
    /// Returns an error for an unsupported datatype or a missing, unexpected, or mismatched
    /// reportable change.
    pub fn try_reported(
        attribute_id: u16,
        configuration: ReportedConfiguration,
    ) -> Result<Self, InvalidReportingRecord> {
        ChangeEncoding::validate(
            configuration.attribute_data_type,
            configuration.reportable_change.as_ref(),
        )?;
        Ok(Self(StatusFields {
            code: SUCCESS,
            direction: REPORTED_ATTRIBUTE,
            attribute_id,
            attribute_data_type: Some(configuration.attribute_data_type),
            minimum_reporting_interval: Some(configuration.minimum_reporting_interval),
            maximum_reporting_interval: Some(configuration.maximum_reporting_interval),
            reportable_change: configuration.reportable_change,
            timeout_period: None,
        }))
    }

    /// Create a successful record for reports received by the target.
    #[must_use]
    pub const fn received(attribute_id: u16, timeout_period: u16) -> Self {
        Self(StatusFields {
            code: SUCCESS,
            direction: RECEIVED_ATTRIBUTE,
            attribute_id,
            attribute_data_type: None,
            minimum_reporting_interval: None,
            maximum_reporting_interval: None,
            reportable_change: None,
            timeout_period: Some(timeout_period),
        })
    }

    /// Create a failed record without configuration fields.
    ///
    /// # Panics
    /// Panics if the status is success or the direction is reserved. Use [`Self::try_failed`] for
    /// raw fields from external inputs.
    #[must_use]
    #[track_caller]
    pub const fn failed(attribute_id: u16, status: u8, direction: u8) -> Self {
        assert!(
            valid_failure(status, direction),
            "valid failed configuration status and direction"
        );
        Self::failure(attribute_id, status, direction)
    }

    /// Validate and construct a failure record, preserving the raw failure status.
    ///
    /// # Errors
    /// Returns an error if the status is success or the direction is reserved.
    pub const fn try_failed(
        attribute_id: u16,
        status: u8,
        direction: u8,
    ) -> Result<Self, InvalidReportingRecord> {
        if !valid_failure(status, direction) {
            return Err(InvalidReportingRecord::InvalidFailure { status, direction });
        }
        Ok(Self::failure(attribute_id, status, direction))
    }

    const fn failure(attribute_id: u16, status: u8, direction: u8) -> Self {
        Self(StatusFields {
            code: status,
            direction,
            attribute_id,
            attribute_data_type: None,
            minimum_reporting_interval: None,
            maximum_reporting_interval: None,
            reportable_change: None,
            timeout_period: None,
        })
    }

    /// Return the complete sender configuration for a successful direction-zero record.
    #[must_use]
    pub fn reported_configuration(&self) -> Option<ReportedConfiguration> {
        Some(ReportedConfiguration {
            attribute_data_type: self.0.attribute_data_type?,
            minimum_reporting_interval: self.0.minimum_reporting_interval?,
            maximum_reporting_interval: self.0.maximum_reporting_interval?,
            reportable_change: self.0.reportable_change.clone(),
        })
    }

    /// Return the timeout for a successful direction-one record.
    #[must_use]
    pub const fn timeout_period(&self) -> Option<u16> {
        self.0.timeout_period
    }

    /// Return the raw status code.
    #[must_use]
    pub const fn status(&self) -> u8 {
        self.0.code
    }

    /// Return the reporting direction.
    #[must_use]
    pub const fn direction(&self) -> u8 {
        self.0.direction
    }

    /// Return the attribute identifier.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.0.attribute_id
    }
}

impl FromLeStream for Status {
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let code = u8::from_le_stream(&mut bytes)?;
        let direction = u8::from_le_stream(&mut bytes)?;
        let attribute_id = u16::from_le_stream(&mut bytes)?;
        match (code, direction) {
            (SUCCESS, REPORTED_ATTRIBUTE) => {
                let attribute_data_type = u8::from_le_stream(&mut bytes)?;
                let minimum_reporting_interval = u16::from_le_stream(&mut bytes)?;
                let maximum_reporting_interval = u16::from_le_stream(&mut bytes)?;
                let reportable_change = match ChangeEncoding::classify(attribute_data_type).ok()? {
                    ChangeEncoding::Discrete => None,
                    ChangeEncoding::Analog => Some(Type::from_le_stream(
                        once(attribute_data_type).chain(bytes),
                    )?),
                };
                Self::try_reported(
                    attribute_id,
                    ReportedConfiguration {
                        attribute_data_type,
                        minimum_reporting_interval,
                        maximum_reporting_interval,
                        reportable_change,
                    },
                )
                .ok()
            }
            (SUCCESS, RECEIVED_ATTRIBUTE) => {
                Some(Self::received(attribute_id, u16::from_le_stream(bytes)?))
            }
            _ => Self::try_failed(attribute_id, code, direction).ok(),
        }
    }
}

impl ToLeStream for Status {
    type Iter = <RecordBytes as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        let fields = self.0;
        let mut bytes = RecordBytes::new();
        bytes.extend([fields.code, fields.direction]);
        bytes.extend(fields.attribute_id.to_le_stream());
        bytes.extend(fields.attribute_data_type.to_le_stream());
        bytes.extend(fields.minimum_reporting_interval.to_le_stream());
        bytes.extend(fields.maximum_reporting_interval.to_le_stream());
        if let Some(change) = fields.reportable_change {
            // The reportable change uses the attribute datatype already present in the record.
            bytes.extend(change.to_le_stream().skip(1));
        }
        bytes.extend(fields.timeout_period.to_le_stream());
        bytes.into_iter()
    }
}

#[cfg(feature = "serde")]
impl StatusFields {
    /// Check that optional fields match the selected wire record layout.
    fn validate(&self) -> Result<(), &'static str> {
        let has_sender_fields = self.attribute_data_type.is_some()
            || self.minimum_reporting_interval.is_some()
            || self.maximum_reporting_interval.is_some()
            || self.reportable_change.is_some();
        match (self.code, self.direction) {
            (SUCCESS, REPORTED_ATTRIBUTE) => {
                let (Some(datatype), Some(_), Some(_), None) = (
                    self.attribute_data_type,
                    self.minimum_reporting_interval,
                    self.maximum_reporting_interval,
                    self.timeout_period,
                ) else {
                    return Err("inconsistent sender reporting fields");
                };
                ChangeEncoding::validate(datatype, self.reportable_change.as_ref())
                    .map_err(|_| "invalid sender reporting datatype or change")
            }
            (SUCCESS, RECEIVED_ATTRIBUTE)
                if !has_sender_fields && self.timeout_period.is_some() =>
            {
                Ok(())
            }
            (code, direction)
                if valid_failure(code, direction)
                    && !has_sender_fields
                    && self.timeout_period.is_none() =>
            {
                Ok(())
            }
            _ => Err("inconsistent reporting status, direction or fields"),
        }
    }
}

/// Failure records carry a non-success status and one of the two defined directions.
const fn valid_failure(status: u8, direction: u8) -> bool {
    status != SUCCESS && matches!(direction, REPORTED_ATTRIBUTE | RECEIVED_ATTRIBUTE)
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = StatusFields::deserialize(deserializer)?;
        fields.validate().map_err(serde::de::Error::custom)?;
        Ok(Self(fields))
    }
}
