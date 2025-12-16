use alloc::vec::Vec;

pub use attribute_report::AttributeReport;
use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

mod attribute_report;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Command {
    reports: Vec<AttributeReport>,
}

impl Command {
    /// Creates a new `Report Attributes` command with the given attribute reports.
    #[must_use]
    pub const fn new(reports: Vec<AttributeReport>) -> Self {
        Self { reports }
    }

    /// Returns the attribute reports of the command.
    #[must_use]
    pub fn reports(&self) -> &[AttributeReport] {
        &self.reports
    }
}

impl Cluster for Command {
    const ID: u16 = 0x0000;
}

impl crate::Command for Command {
    const ID: u8 = 0x0A;
    const DIRECTION: zigbee::Direction = zigbee::Direction::ServerToClient;
    const TYPE: crate::Type = crate::Type::Global;
}
