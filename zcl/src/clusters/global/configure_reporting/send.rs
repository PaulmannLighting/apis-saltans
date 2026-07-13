//! Command for configuring a device to send attribute reports.

use std::boxed::Box;

use le_stream::{FromLeStream, ToLeStream};
use zb_core::Direction;
use zb_core::types::Type;

use crate::macros::zcl_command;

const DIRECTION: Direction = Direction::ClientToServer;

/// Configuration for an attribute that the target device shall report.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct AttributeReportingConfiguration {
    direction: u8,
    attribute_id: u16,
    attribute_data_type: u8,
    minimum_reporting_interval: u16,
    maximum_reporting_interval: u16,
    reportable_change: Option<Type>,
}

impl AttributeReportingConfiguration {
    /// Creates a configuration for an attribute that the target device shall report.
    #[must_use]
    pub const fn new(
        attribute_id: u16,
        attribute_data_type: u8,
        minimum_reporting_interval: u16,
        maximum_reporting_interval: u16,
        reportable_change: Option<Type>,
    ) -> Self {
        Self {
            direction: DIRECTION as u8,
            attribute_id,
            attribute_data_type,
            minimum_reporting_interval,
            maximum_reporting_interval,
            reportable_change,
        }
    }

    /// Returns the attribute ID.
    #[must_use]
    pub const fn attribute_id(&self) -> u16 {
        self.attribute_id
    }

    /// Returns the attribute data type.
    #[must_use]
    pub const fn attribute_data_type(&self) -> u8 {
        self.attribute_data_type
    }

    /// Returns the minimum reporting interval.
    #[must_use]
    pub const fn minimum_reporting_interval(&self) -> u16 {
        self.minimum_reporting_interval
    }

    /// Returns the maximum reporting interval.
    #[must_use]
    pub const fn maximum_reporting_interval(&self) -> u16 {
        self.maximum_reporting_interval
    }

    /// Returns the reportable change for an analog attribute.
    #[must_use]
    pub const fn reportable_change(&self) -> Option<&Type> {
        self.reportable_change.as_ref()
    }
}

zcl_command! {
    /// Command that configures the target to send attribute reports to its bindings.
    Command {
        Global;
        command_id: super::COMMAND_ID;
        direction: DIRECTION;
        conversions: manual;
        fields {
            attributes: Box<[AttributeReportingConfiguration]>,
        }

        getters {
            /// Returns the attribute reporting configurations.
            #[must_use]
            pub const fn attributes(&self) -> &[AttributeReportingConfiguration] {
                &self.attributes
            }
        }
    }
}
