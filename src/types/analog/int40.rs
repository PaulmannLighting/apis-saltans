use intx::I40;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: [u8; 5] = [0x80, 0x00, 0x00, 0x00, 0x00]; // big-endian representation of 0x80_0000_0000

/// The `40-bit signed integer` type, short `int40`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int40(I40);

impl Int40 {
    /// Crate a new `Int40` from an `I40` value.
    #[must_use]
    pub fn new(value: I40) -> Option<Self> {
        if value == I40::from_be_bytes(NON_VALUE) {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int40` with the non-value.
    #[must_use]
    pub fn non_value(self) -> Self {
        Self(I40::from_be_bytes(NON_VALUE))
    }
}

impl From<Int40> for Option<I40> {
    fn from(value: Int40) -> Self {
        if value.0 == I40::from_be_bytes(NON_VALUE) {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Int40> for Option<i64> {
    fn from(value: Int40) -> Self {
        Option::<I40>::from(value).map(Into::into)
    }
}

impl TryFrom<I40> for Int40 {
    type Error = ();

    fn try_from(value: I40) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<i64> for Int40 {
    type Error = Option<i64>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        I40::try_from(value).map_or(Err(Some(value)), |i40| Self::new(i40).ok_or(None))
    }
}
