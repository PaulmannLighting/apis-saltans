use macaddr::MacAddr8;
use repr_discriminant::ReprDiscriminant;

/// Address type for Bind Request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ReprDiscriminant)]
#[repr(u8)]
pub enum Destination {
    /// 16-bit group address.
    Group(u16) = 0x01,
    /// 64-bit extended address.
    Extended {
        /// 64-bit MAC address.
        address: MacAddr8,
        /// Endpoint on the destination device.
        endpoint: u8,
    } = 0x03,
}
