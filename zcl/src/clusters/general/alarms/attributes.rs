//! Attributes for the Alarms cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;

/// Readable attributes for the Alarms cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// Amount of alarms currently present in the alarms table.
    AlarmCount(u16) = 0x0000, // Valid range `0x00` to `0xff`.
}
