/// Fragmentation field of the APS frame header extended control field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Fragmentation {
    /// Frame is not fragmented.
    None = 0b00,

    /// First fragment of a fragmented frame.
    First {
        /// Total number of payload blocks in the fragmented transmission.
        blocks: u8,
    } = 0b01,

    /// Follow-up fragment of a fragmented frame.
    Followup {
        /// Follow-up fragment index carried in the block number field.
        index: u8,
    } = 0b10,
}
