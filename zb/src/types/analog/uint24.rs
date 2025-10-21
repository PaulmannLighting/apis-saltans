use intx::U24;
use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: U24 = U24::MAX;

/// The `24-bit unsigned integer` type, short `uint24`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint24(U24);

impl From<Uint24> for Option<U24> {
    fn from(value: Uint24) -> Self {
        if value.0 == NON_VALUE {
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
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<U24>> for Uint24 {
    type Error = ();

    fn try_from(value: Option<U24>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Ok(Self(NON_VALUE)), Self::try_from)
    }
}

impl TryFrom<u32> for Uint24 {
    type Error = Option<u32>;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        U24::try_from(value).map_or(Err(Some(value)), |u24| {
            Self::try_from(u24).map_err(|()| None)
        })
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
