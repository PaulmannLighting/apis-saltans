//! Reportable attributes for the Level cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::Uint8;

use super::read;

/// Reportable attributes for the Level cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// Current level of the device.
    CurrentLevel(Uint8) = 0x0000,
    /// Current frequency of the device.
    CurrentFrequency(u16) = 0x0004,
}

impl TryFrom<read::Attribute> for Attribute {
    type Error = read::Attribute;

    fn try_from(read: read::Attribute) -> Result<Self, Self::Error> {
        match read {
            read::Attribute::CurrentLevel(level) => Ok(Self::CurrentLevel(level)),
            read::Attribute::CurrentFrequency(freq) => Ok(Self::CurrentFrequency(freq)),
            other => Err(other),
        }
    }
}
