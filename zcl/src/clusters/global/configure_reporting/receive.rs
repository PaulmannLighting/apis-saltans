//! Command for configuring a device to receive attribute reports.

use std::boxed::Box;

use le_stream::{FromLeStream, ToLeStream};
use zb_core::Direction;

use crate::macros::zcl_command;

const DIRECTION: Direction = Direction::ServerToClient;

/// Configuration for an attribute report that the target device shall receive.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct AttributeReportingConfiguration {
    direction: u8,
    attribute_id: u16,
    timeout_period: u16,
}

impl AttributeReportingConfiguration {
    /// Creates a configuration for an attribute report that the target device shall receive.
    #[must_use]
    pub const fn new(attribute_id: u16, timeout_period: u16) -> Self {
        Self {
            direction: DIRECTION as u8,
            attribute_id,
            timeout_period,
        }
    }

    /// Returns the attribute ID.
    #[must_use]
    pub const fn attribute_id(self) -> u16 {
        self.attribute_id
    }

    /// Returns the timeout period.
    #[must_use]
    pub const fn timeout_period(self) -> u16 {
        self.timeout_period
    }
}

zcl_command! {
    /// Command that configures the target to receive attribute reports.
    Command {
        Global;
        command_id: super::COMMAND_ID;
        direction: DIRECTION;
        conversions: manual;
        fields {
            attributes: Box<[AttributeReportingConfiguration]>,
        }

        getters {
            /// Returns the received-attribute reporting configurations.
            #[must_use]
            pub const fn attributes(&self) -> &[AttributeReportingConfiguration] {
                &self.attributes
            }
        }
    }
}
