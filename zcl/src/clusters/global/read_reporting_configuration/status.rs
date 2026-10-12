use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::Type;

const SUCCESS: u8 = 0x00;
const REPORTED_ATTRIBUTE: u8 = 0x00;
const RECEIVED_ATTRIBUTE: u8 = 0x01;

/// Parameters of an attribute reported by the target.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReportedConfiguration {
    /// ZCL wire type of the attribute.
    pub attribute_data_type: u8,
    /// Minimum time between reports, in seconds.
    pub minimum_reporting_interval: u16,
    /// Maximum time between reports, in seconds.
    pub maximum_reporting_interval: u16,
    /// Change threshold for analog attributes; absent for discrete attributes.
    pub reportable_change: Option<Type>,
}

/// Attribute reporting configuration record of a Read Reporting Configuration response.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Status {
    code: u8,
    direction: u8,
    attribute_id: u16,
    attribute_data_type: Option<u8>,
    minimum_reporting_interval: Option<u16>,
    maximum_reporting_interval: Option<u16>,
    reportable_change: Option<Type>,
    timeout_period: Option<u16>,
}

impl Status {
    /// Create a successful record for an attribute reported by the target.
    #[must_use]
    pub fn reported(attribute_id: u16, configuration: ReportedConfiguration) -> Self {
        Self {
            code: SUCCESS,
            direction: REPORTED_ATTRIBUTE,
            attribute_id,
            attribute_data_type: Some(configuration.attribute_data_type),
            minimum_reporting_interval: Some(configuration.minimum_reporting_interval),
            maximum_reporting_interval: Some(configuration.maximum_reporting_interval),
            reportable_change: configuration.reportable_change,
            timeout_period: None,
        }
    }

    /// Create a successful record for reports received by the target.
    #[must_use]
    pub const fn received(attribute_id: u16, timeout_period: u16) -> Self {
        Self {
            code: SUCCESS,
            direction: RECEIVED_ATTRIBUTE,
            attribute_id,
            attribute_data_type: None,
            minimum_reporting_interval: None,
            maximum_reporting_interval: None,
            reportable_change: None,
            timeout_period: Some(timeout_period),
        }
    }

    /// Create a failed record without optional configuration fields.
    ///
    /// Raw status and direction values allow protocol extensions to be represented.
    #[must_use]
    pub const fn failed(attribute_id: u16, status: u8, direction: u8) -> Self {
        Self {
            code: status,
            direction,
            attribute_id,
            attribute_data_type: None,
            minimum_reporting_interval: None,
            maximum_reporting_interval: None,
            reportable_change: None,
            timeout_period: None,
        }
    }

    /// Return the raw status code.
    #[must_use]
    pub const fn status(&self) -> u8 {
        self.code
    }

    /// Return the reporting direction.
    #[must_use]
    pub const fn direction(&self) -> u8 {
        self.direction
    }

    /// Return the attribute identifier.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }
}

#[cfg(test)]
mod tests {
    use le_stream::ToLeStream;
    use zb_core::types::{Type, Uint16};

    use super::{ReportedConfiguration, Status};

    const ATTRIBUTE_ID: u16 = 0x1234;
    const MINIMUM: u16 = 10;
    const MAXIMUM: u16 = 60;
    const TIMEOUT: u16 = 300;
    const FAILURE: u8 = 0x86;
    const REPORTED: u8 = 0;
    const RECEIVED: u8 = 1;
    const UINT16: u8 = 0x21;
    const BOOLEAN: u8 = 0x10;
    const CHANGE: Uint16 = Uint16::new(2);

    #[test]
    fn reported_configuration_retains_existing_record_encoding() {
        let analog = Status::reported(
            ATTRIBUTE_ID,
            ReportedConfiguration {
                attribute_data_type: UINT16,
                minimum_reporting_interval: MINIMUM,
                maximum_reporting_interval: MAXIMUM,
                reportable_change: Some(Type::Uint16(CHANGE)),
            },
        );
        assert_eq!(analog.direction(), REPORTED);
        assert_eq!(
            analog.to_le_stream().collect::<Vec<_>>(),
            [0, 0, 0x34, 0x12, 0x21, 10, 0, 60, 0, 0x21, 2, 0]
        );
        let discrete = Status::reported(
            ATTRIBUTE_ID,
            ReportedConfiguration {
                attribute_data_type: BOOLEAN,
                minimum_reporting_interval: MINIMUM,
                maximum_reporting_interval: MAXIMUM,
                reportable_change: None,
            },
        );
        assert_eq!(
            discrete.to_le_stream().collect::<Vec<_>>(),
            [0, 0, 0x34, 0x12, 0x10, 10, 0, 60, 0]
        );
    }

    #[test]
    fn received_configuration_contains_only_the_timeout() {
        let record = Status::received(ATTRIBUTE_ID, TIMEOUT);
        assert_eq!(record.direction(), RECEIVED);
        assert_eq!(record.attribute_id(), ATTRIBUTE_ID);
        assert_eq!(
            record.to_le_stream().collect::<Vec<_>>(),
            [0, 1, 0x34, 0x12, 0x2c, 1]
        );
    }

    #[test]
    fn failed_configuration_contains_no_optional_fields() {
        let record = Status::failed(ATTRIBUTE_ID, FAILURE, RECEIVED);
        assert_eq!(record.status(), FAILURE);
        assert_eq!(
            record.to_le_stream().collect::<Vec<_>>(),
            [0x86, 1, 0x34, 0x12]
        );
    }
}
