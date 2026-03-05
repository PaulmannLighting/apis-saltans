//! Reportable attributes for the Level cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::Uint8;

use super::readable;

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

impl TryFrom<readable::Attribute> for Attribute {
    type Error = readable::Attribute;

    fn try_from(read: readable::Attribute) -> Result<Self, Self::Error> {
        match read {
            readable::Attribute::CurrentLevel(level) => Ok(Self::CurrentLevel(level)),
            readable::Attribute::CurrentFrequency(freq) => Ok(Self::CurrentFrequency(freq)),
            other => Err(other),
        }
    }
}
