use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::Type;

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
    /// Create a reporting-configuration response record.
    #[expect(
        clippy::too_many_arguments,
        reason = "the constructor mirrors the ZCL reporting-configuration record"
    )]
    #[must_use]
    pub const fn new(
        status: u8,
        direction: u8,
        attribute_id: u16,
        attribute_data_type: Option<u8>,
        minimum_reporting_interval: Option<u16>,
        maximum_reporting_interval: Option<u16>,
        reportable_change: Option<Type>,
        timeout_period: Option<u16>,
    ) -> Self {
        Self {
            code: status,
            direction,
            attribute_id,
            attribute_data_type,
            minimum_reporting_interval,
            maximum_reporting_interval,
            reportable_change,
            timeout_period,
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
