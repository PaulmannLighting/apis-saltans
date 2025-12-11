use std::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Pan ID Conflict Report TLV.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct PanIdConflictReport {
    nwk_pan_id_conflict_count: u16,
}

impl PanIdConflictReport {
    /// Create a new `PanIdConflictReport`.
    #[must_use]
    pub const fn new(nwk_pan_id_conflict_count: u16) -> Self {
        Self {
            nwk_pan_id_conflict_count,
        }
    }

    /// Get the Network PAN ID Conflict Count.
    #[must_use]
    pub const fn nwk_pan_id_conflict_count(self) -> u16 {
        self.nwk_pan_id_conflict_count
    }
}

impl Tag for PanIdConflictReport {
    const TAG: u8 = 66;

    fn size(&self) -> usize {
        2
    }
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

impl ToLeStream for PanIdConflictReport {
    type Iter =
        Chain<Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>, <u16 as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.nwk_pan_id_conflict_count.to_le_stream())
    }
}
