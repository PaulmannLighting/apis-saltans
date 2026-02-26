//! Report Attributes Command.

use alloc::boxed::Box;

use le_stream::{FromLeStream, ToLeStream};

pub use self::attribute_report::AttributeReport;
use crate::Global;

mod attribute_report;

/// Report Attributes Command.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Command {
    reports: Box<[AttributeReport]>,
}

impl Command {
    /// Creates a new `Report Attributes` command with the given attribute reports.
    #[must_use]
    pub const fn new(reports: Box<[AttributeReport]>) -> Self {
        Self { reports }
    }

    /// Returns the attribute reports of the command.
    #[must_use]
    pub fn reports(&self) -> &[AttributeReport] {
        &self.reports
    }
}

impl Global for Command {
    const ID: u8 = 0x0A;
    const DIRECTION: zigbee::Direction = zigbee::Direction::ServerToClient;
}
