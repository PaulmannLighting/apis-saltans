use intx::U40;
use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: U40 = U40::MAX;

/// The `40-bit unsigned integer` type, short `uint40`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint40(U40);

impl From<Uint40> for Option<U40> {
    fn from(value: Uint40) -> Self {
        value.try_into().ok()
    }
}

impl From<Uint40> for Option<u64> {
    fn from(value: Uint40) -> Self {
        Option::<U40>::from(value).map(Into::into)
    }
}

impl TryFrom<Uint40> for U40 {
    type Error = ();

    fn try_from(value: Uint40) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<U40> for Uint40 {
    type Error = ();

    fn try_from(value: U40) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<U40>> for Uint40 {
    type Error = ();

    fn try_from(value: Option<U40>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Ok(Self(NON_VALUE)), Self::try_from)
    }
}

impl TryFrom<u64> for Uint40 {
    type Error = Option<u64>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        U40::try_from(value).map_or(Err(Some(value)), |u40| {
            Self::try_from(u40).map_err(|()| None)
        })
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
        serde::Deserialize::deserialize(deserializer)
            .map(U40::from_be_bytes)
            .map(Self)
    }
}
