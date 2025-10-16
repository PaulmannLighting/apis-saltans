use intx::U48;
use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: U48 = U48::MAX;

/// The `48-bit unsigned integer` type, short `uint48`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Uint48(U48);

impl From<Uint48> for Option<U48> {
    fn from(value: Uint48) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl From<Uint48> for Option<u64> {
    fn from(value: Uint48) -> Self {
        Option::<U48>::from(value).map(Into::into)
    }
}

impl TryFrom<U48> for Uint48 {
    type Error = ();

    fn try_from(value: U48) -> Result<Self, Self::Error> {
        if value == NON_VALUE {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<U48>> for Uint48 {
    type Error = ();

    fn try_from(value: Option<U48>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Ok(Self(NON_VALUE)), Self::try_from)
    }
}

impl TryFrom<u64> for Uint48 {
    type Error = Option<u64>;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        U48::try_from(value).map_or(Err(Some(value)), |u48| {
            Self::try_from(u48).map_err(|()| None)
        })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Uint48 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Uint48 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 6] = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self(U48::from_be_bytes(bytes)))
    }
}
