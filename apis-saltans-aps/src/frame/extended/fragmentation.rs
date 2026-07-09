use num_derive::FromPrimitive;

/// Fragmentation field of the APS frame header extended control field
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Fragmentation {
    /// Frame is not fragmented.
    None = 0b00,

    /// First fragment of a fragmented frame.
    First {
        blocks: u8,
    } = 0b01,

    Followup {
        index: u8,
    } = 0b10,
}
