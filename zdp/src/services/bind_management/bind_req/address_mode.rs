use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(
    Clone, Copy, Debug, Eq, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[repr(u8)]
pub enum AddressMode {
    /// Group Addressing Mode
    Group = 0x01,
    /// Extended Addressing Mode
    Extended = 0x03,
}
