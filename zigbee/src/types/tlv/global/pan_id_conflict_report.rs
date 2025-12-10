use le_stream::FromLeStream;

use crate::types::tlv::Tag;

/// Pan ID Conflict Report TLV.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct PanIdConflictReport {
    nwk_pan_id_conflict_count: u16,
}

impl PanIdConflictReport {
    /// Get the Network PAN ID Conflict Count.
    #[must_use]
    pub const fn nwk_pan_id_conflict_count(self) -> u16 {
        self.nwk_pan_id_conflict_count
    }
}

impl Tag for PanIdConflictReport {
    const TAG: u8 = 66;
}

impl From<PanIdConflictReport> for u16 {
    fn from(value: PanIdConflictReport) -> Self {
        value.nwk_pan_id_conflict_count
    }
}

impl From<u16> for PanIdConflictReport {
    fn from(nwk_pan_id_conflict_count: u16) -> Self {
        Self {
            nwk_pan_id_conflict_count,
        }
    }
}
