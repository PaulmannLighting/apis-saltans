use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Type;

/// Configuration for attribute reporting.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct AttributeReportingConfiguration {
    direction: u8,
    attribute_id: u16,
    attribute_data_type: Option<u8>,
    minimum_reporting_interval: Option<u16>,
    maximum_reporting_interval: Option<u16>,
    reportable_change: Option<Type>,
    timeout_period: Option<u16>,
}

impl AttributeReportingConfiguration {
    /// Creates a new `AttributeReportingConfiguration`.
    #[must_use]
    pub const fn new(
        direction: u8,
        attribute_id: u16,
        attribute_data_type: Option<u8>,
        minimum_reporting_interval: Option<u16>,
        maximum_reporting_interval: Option<u16>,
        reportable_change: Option<Type>,
        timeout_period: Option<u16>,
    ) -> Self {
        Self {
            direction,
            attribute_id,
            attribute_data_type,
            minimum_reporting_interval,
            maximum_reporting_interval,
            reportable_change,
            timeout_period,
        }
    }

    /// Returns the direction.
    #[must_use]
    pub const fn direction(&self) -> u8 {
        self.direction
    }

    /// Returns the attribute ID.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }

    /// Returns the attribute data type.
    #[must_use]
    pub const fn attribute_data_type(&self) -> Option<u8> {
        self.attribute_data_type
    }

    /// Returns the minimum reporting interval.
    #[must_use]
    pub const fn minimum_reporting_interval(&self) -> Option<u16> {
        self.minimum_reporting_interval
    }

    /// Returns the maximum reporting interval.
    #[must_use]
    pub const fn maximum_reporting_interval(&self) -> Option<u16> {
        self.maximum_reporting_interval
    }

    /// Returns the reportable change.
    #[must_use]
    pub const fn reportable_change(&self) -> Option<&Type> {
        self.reportable_change.as_ref()
    }

    /// Returns the timeout period.
    #[must_use]
    pub const fn timeout_period(&self) -> Option<u16> {
        self.timeout_period
    }
}
