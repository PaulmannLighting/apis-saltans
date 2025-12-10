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
