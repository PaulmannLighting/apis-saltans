use le_stream::derive::{FromLeStream, ToLeStream};

use crate::zcl::lighting::color_control::DriftCompensation;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct MaybeDriftCompensation(u8);

impl From<DriftCompensation> for MaybeDriftCompensation {
    fn from(value: DriftCompensation) -> Self {
        Self(value as u8)
    }
}

impl TryFrom<MaybeDriftCompensation> for DriftCompensation {
    type Error = u8;

    fn try_from(value: MaybeDriftCompensation) -> Result<Self, Self::Error> {
        Self::try_from(value.0)
    }
}
