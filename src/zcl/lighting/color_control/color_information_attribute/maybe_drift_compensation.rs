use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::DriftCompensation;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct MaybeDriftCompensation(u8);

impl MaybeDriftCompensation {
    /// Returns the inner value.
    ///
    /// # Errors
    ///
    /// If the inner value does not correspond to a valid `DriftCompensation`, this will return an error.
    pub fn value(self) -> Result<DriftCompensation, u8> {
        DriftCompensation::try_from(self.0)
    }
}

impl From<DriftCompensation> for MaybeDriftCompensation {
    fn from(value: DriftCompensation) -> Self {
        Self(value as u8)
    }
}
