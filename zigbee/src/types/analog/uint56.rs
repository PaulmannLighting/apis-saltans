use intx::U56;
use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: U56 = U56::MAX;

/// The `56-bit unsigned integer` type, short `uint56`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint56(U56);

impl From<Uint56> for Option<U56> {
    fn from(value: Uint56) -> Self {
        value.try_into().ok()
    }
}

impl From<Uint56> for Option<u64> {
    fn from(value: Uint56) -> Self {
        Option::<U56>::from(value).map(Into::into)
    }
}

impl TryFrom<Uint56> for U56 {
    type Error = ();

    fn try_from(value: Uint56) -> Result<Self, Self::Error> {
        if value.0 == NON_VALUE {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<U56> for Uint56 {
    type Error = ();

    fn try_from(value: U56) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<U56>> for Uint56 {
    type Error = ();

    fn try_from(value: Option<U56>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Ok(Self(NON_VALUE)), Self::try_from)
    }
}

impl TryFrom<u64> for Uint56 {
    type Error = Option<u64>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        U56::try_from(value).map_or(Err(Some(value)), |u56| {
            Self::try_from(u56).map_err(|()| None)
        })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Uint56 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Uint56 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 7] = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self(U56::from_be_bytes(bytes)))
    }
}
