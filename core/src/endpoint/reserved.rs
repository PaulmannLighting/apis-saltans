/// A Zigbee reserved endpoint ID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct Reserved(pub(crate) u8);

impl Reserved {
    /// The minimum valid reserved endpoint ID.
    pub const MIN_ID: u8 = 241;

    /// The maximum valid reserved endpoint ID.
    pub const MAX_ID: u8 = 254;
}

impl_fmt_via_value!(Reserved, u8, |value| value.0);

impl From<Reserved> for u8 {
    fn from(endpoint: Reserved) -> Self {
        endpoint.0
    }
}
