//! Alarms table implementation.

use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::{Uint8, Uint32};

/// An entry in the Alarms cluster's table.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream)]
pub struct Entry {
    code: Uint8,
    cluster_id: u16,
    timestamp: Uint32,
}

impl Entry {
    /// Creates a new entry with the given code and cluster ID, and a timestamp of 0.
    #[must_use]
    pub const fn new(code: Uint8, cluster_id: u16, timestamp: Uint32) -> Self {
        Self {
            code,
            cluster_id,
            timestamp,
        }
    }

    /// Returns the alarm code.
    pub const fn code(&self) -> Uint8 {
        self.code
    }

    /// Returns the cluster ID associated with the alarm.
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Returns the timestamp of when the alarm was triggered.
    pub const fn timestamp(&self) -> Uint32 {
        // TODO: Is this really a `Uint32` or actually a `UtcTime`?
        self.timestamp
    }

    /// Returns the Alarm code.
    pub fn alarm(&self) -> Result<(), (Uint8, u16)> {
        todo!(
            "Return a variant of a generic Alarm enum based on the code and cluster ID, \
            or an error if the code and cluster ID do not correspond to a known alarm."
        )
    }
}
