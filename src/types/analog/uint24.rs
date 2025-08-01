use intx::U24;
use le_stream::derive::{FromLeStream, ToLeStream};

/// The `24-bit unsigned integer` type, short `uint24`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint24(U24);

impl Uint24 {
    /// The non-value. See Table 2-11.
    pub const NON_VALUE: U24 = U24::MAX;

    /// Crate a new `Uint24` from an `U24` value.
    #[must_use]
    pub fn new(value: U24) -> Option<Self> {
        if value == Self::NON_VALUE {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Create a new `Uint24` with the non-value.
    #[must_use]
    pub const fn non_value() -> Self {
        Self(Self::NON_VALUE)
    }
}

impl From<Uint24> for Option<U24> {
    fn from(value: Uint24) -> Self {
        if value.0 == Uint24::NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Uint24> for Option<u32> {
    fn from(value: Uint24) -> Self {
        Option::<U24>::from(value).map(Into::into)
    }
}

impl TryFrom<U24> for Uint24 {
    type Error = ();

    fn try_from(value: U24) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<u32> for Uint24 {
    type Error = Option<u32>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        U24::try_from(value).map_or(Err(Some(value)), |u24| Self::new(u24).ok_or(None))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Uint24 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Uint24 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 3] = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self(U24::from_be_bytes(bytes)))
    }
}
