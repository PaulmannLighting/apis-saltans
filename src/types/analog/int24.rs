use intx::I24;
use le_stream::derive::{FromLeStream, ToLeStream};

/// See Table 2-11.
const NON_VALUE: [u8; 3] = [0x80, 0x00, 0x00]; // big-endian representation of 0x800000

/// The `24-bit signed integer` type, short `int24`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int24(I24);

impl Int24 {
    /// Crate a new `Int24` from an `I24` value.
    #[must_use]
    pub fn new(value: I24) -> Option<Self> {
        if value == I24::from_be_bytes(NON_VALUE) {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int24` with the non-value.
    #[must_use]
    pub fn non_value(self) -> Self {
        Self(I24::from_be_bytes(NON_VALUE))
    }
}

impl From<Int24> for Option<I24> {
    fn from(value: Int24) -> Self {
        if value.0 == I24::from_be_bytes(NON_VALUE) {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Int24> for Option<i32> {
    fn from(value: Int24) -> Self {
        Option::<I24>::from(value).map(Into::into)
    }
}

impl TryFrom<I24> for Int24 {
    type Error = ();

    fn try_from(value: I24) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<i32> for Int24 {
    type Error = Option<i32>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        I24::try_from(value).map_or(Err(Some(value)), |i24| Self::new(i24).ok_or(None))
    }
}
