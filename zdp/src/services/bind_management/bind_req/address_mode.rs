use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromPrimitive)]
#[repr(u8)]
pub enum AddressMode {
    /// Group Addressing Mode
    Group = 0x01,
    /// Extended Addressing Mode
    Extended = 0x03,
}
