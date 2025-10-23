use le_stream::derive::{FromLeStream, ToLeStream};
use zigbee::types::Uint16;

const MAX_GROUP: u16 = 0xfff7;

/// An identifier for the current group.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct CurrentGroup(Uint16);

impl TryFrom<Uint16> for CurrentGroup {
    type Error = Uint16;

    fn try_from(value: Uint16) -> Result<Self, Self::Error> {
        if value <= Uint16::new(MAX_GROUP) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

impl TryFrom<u16> for CurrentGroup {
    type Error = Option<u16>;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match Uint16::try_from(value) {
            Ok(uint16) => Self::try_from(uint16).map_err(|_| Some(value)),
            Err(()) => Err(None),
        }
    }
}
