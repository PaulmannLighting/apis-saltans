use intx::I48;
use le_stream::derive::{FromLeStream, ToLeStream};

/// The `48-bit signed integer` type, short `int48`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int48(I48);

impl Int48 {
    /// The non-value. See Table 2-11.
    ///
    /// Big-endian representation of `0x8000_0000_0000`.
    pub const NON_VALUE: [u8; 6] = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00];

    /// Crate a new `Int48` from an `I48` value.
    #[must_use]
    pub fn new(value: I48) -> Option<Self> {
        if value == I48::from_be_bytes(Self::NON_VALUE) {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Int48` with the non-value.
    #[must_use]
    pub fn non_value() -> Self {
        Self(I48::from_be_bytes(Self::NON_VALUE))
    }
}

impl From<Int48> for Option<I48> {
    fn from(value: Int48) -> Self {
        if value == Int48::non_value() {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Int48> for Option<i64> {
    fn from(value: Int48) -> Self {
        Option::<I48>::from(value).map(Into::into)
    }
}

impl TryFrom<I48> for Int48 {
    type Error = ();

    fn try_from(value: I48) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<i64> for Int48 {
    type Error = Option<i64>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        I48::try_from(value).map_or(Err(Some(value)), |i48| Self::new(i48).ok_or(None))
    }
}
