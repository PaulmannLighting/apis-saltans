//! Command for configuring a device to send attribute reports.

use std::boxed::Box;

use bytes::Bytes;
use le_stream::{FromLeStream, ToLeStream};
use zb_core::{Direction, TypeId};

use crate::attributes::{Analog, Discrete};
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
    reportable_change: Bytes,
}

impl AttributeReportingConfiguration {
    /// Creates a reporting configuration for an analog attribute.
    #[must_use]
    pub(crate) fn analog<T>(attribute_id: u16, analog: Analog<T>) -> Self
    where
        T: TypeId + ToLeStream,
    {
        Self {
            direction: DIRECTION as u8,
            attribute_id,
            attribute_data_type: T::ID,
            minimum_reporting_interval: analog.minimum_reporting_interval,
            maximum_reporting_interval: analog.maximum_reporting_interval,
            reportable_change: analog.reportable_change.to_le_stream().collect(),
        }
    }

    /// Creates a reporting configuration for a discrete attribute.
    #[must_use]
    pub(crate) const fn discrete<T>(attribute_id: u16, discrete: &Discrete<T>) -> Self
    where
        T: TypeId,
    {
        Self {
            direction: DIRECTION as u8,
            attribute_id,
            attribute_data_type: T::ID,
            minimum_reporting_interval: discrete.minimum_reporting_interval,
            maximum_reporting_interval: discrete.maximum_reporting_interval,
            reportable_change: Bytes::new(),
        }
    }
}

zcl_command! {
    /// Command that configures the target to send attribute reports to its bindings.
    Command {
        Global;
        command_id: super::COMMAND_ID;
        direction: DIRECTION;
        response: super::Response;
        => crate::global::ConfigureReportingSend;
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
