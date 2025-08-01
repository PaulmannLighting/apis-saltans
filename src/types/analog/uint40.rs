use intx::U40;
use le_stream::derive::{FromLeStream, ToLeStream};

/// The `40-bit unsigned integer` type, short `uint40`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint40(U40);

impl Uint40 {
    /// The non-value. See Table 2-11.
    pub const NON_VALUE: U40 = U40::MAX;

    /// Crate a new `Uint40` from an `U40` value.
    #[must_use]
    pub fn new(value: U40) -> Option<Self> {
        if value == Self::NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Uint40` with the non-value.
    #[must_use]
    pub const fn non_value() -> Self {
        Self(Self::NON_VALUE)
    }
}

impl From<Uint40> for Option<U40> {
    fn from(value: Uint40) -> Self {
        if value.0 == Uint40::NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Uint40> for Option<u64> {
    fn from(value: Uint40) -> Self {
        Option::<U40>::from(value).map(Into::into)
    }
}

impl TryFrom<U40> for Uint40 {
    type Error = ();

    fn try_from(value: U40) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<u64> for Uint40 {
    type Error = Option<u64>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        U40::try_from(value).map_or(Err(Some(value)), |u40| Self::new(u40).ok_or(None))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Uint40 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Uint40 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 5] = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self(U40::from_be_bytes(bytes)))
    }
}
