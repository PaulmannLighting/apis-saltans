/// A reserved Zigbee endpoint ID.
///
/// IDs in this range are preserved when a raw protocol value cannot be represented as a valid
/// [`super::Endpoint`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct Reserved(pub(super) u8);

impl Reserved {
    /// The minimum reserved endpoint ID.
    pub const MIN_ID: u8 = 0xF1;

    /// The maximum reserved endpoint ID.
    pub const MAX_ID: u8 = 0xFE;

    /// Return the raw reserved endpoint ID.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl_fmt_via_value!(Reserved, u8, |value| value.as_u8());

impl From<Reserved> for u8 {
    fn from(endpoint: Reserved) -> Self {
        endpoint.as_u8()
    }
}
