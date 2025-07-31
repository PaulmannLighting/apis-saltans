use intx::I56;
use le_stream::derive::{FromLeStream, ToLeStream};

/// The `56-bit signed integer` type, short `int56`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int56(I56);

impl Int56 {
    /// The non-value. See Table 2-11.
    ///
    /// Big-endian representation of `0x80_0000_0000_0000`.
    pub const NON_VALUE: [u8; 7] = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    /// Crate a new `Int56` from an `I56` value.
    #[must_use]
    pub fn new(value: I56) -> Option<Self> {
        if value == I56::from_be_bytes(Self::NON_VALUE) {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int56` with the non-value.
    #[must_use]
    pub fn non_value() -> Self {
        Self(I56::from_be_bytes(Self::NON_VALUE))
    }
}

impl From<Int56> for Option<I56> {
    fn from(value: Int56) -> Self {
        if value == Int56::non_value() {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Int56> for Option<i64> {
    fn from(value: Int56) -> Self {
        Option::<I56>::from(value).map(Into::into)
    }
}

impl TryFrom<I56> for Int56 {
    type Error = ();

    fn try_from(value: I56) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<i64> for Int56 {
    type Error = Option<i64>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        I56::try_from(value).map_or(Err(Some(value)), |i56| Self::new(i56).ok_or(None))
    }
}
