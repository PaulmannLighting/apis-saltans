//! Reporting configuration command for the Global cluster.

use alloc::vec::Vec;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Direction;

pub use self::attribute_reporting_configuration::AttributeReportingConfiguration;
pub use self::attribute_status::AttributeStatus;
use crate::Global;

mod attribute_reporting_configuration;
mod attribute_status;

/// Command to configure reporting for attributes.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Command {
    attributes: Vec<AttributeReportingConfiguration>,
}

impl Command {
    /// Creates a new `Command`.
    #[must_use]
    pub const fn new(attributes: Vec<AttributeReportingConfiguration>) -> Self {
        Self { attributes }
    }

    /// Returns the attributes.
    #[must_use]
    pub fn attributes(&self) -> &[AttributeReportingConfiguration] {
        &self.attributes
    }
}

impl Global for Command {
    const ID: u8 = 0x06;
    const DIRECTION: Direction = Direction::ClientToServer;
}

/// Status of an attribute reporting configuration.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Response {
    status: Vec<AttributeStatus>,
}

impl Response {
    /// Creates a new `Response`.
    #[must_use]
    pub const fn new(status: Vec<AttributeStatus>) -> Self {
        Self { status }
    }

    /// Returns the status.
    #[must_use]
    pub fn status(&self) -> &[AttributeStatus] {
        &self.status
    }
}

impl Global for Response {
    const ID: u8 = 0x07;
    const DIRECTION: Direction = Direction::ServerToClient;
}
