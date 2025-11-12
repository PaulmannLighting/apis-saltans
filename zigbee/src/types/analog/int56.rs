use intx::I56;
use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE_BE: [u8; 7] = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

/// The `56-bit signed integer` type, short `int56`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int56(I56);

impl From<Int56> for Option<I56> {
    fn from(value: Int56) -> Self {
        value.try_into().ok()
    }
}

impl From<Int56> for Option<i64> {
    fn from(value: Int56) -> Self {
        Option::<I56>::from(value).map(Into::into)
    }
}

impl TryFrom<Int56> for I56 {
    type Error = ();

    fn try_from(value: Int56) -> Result<Self, Self::Error> {
        if value.0 == Self::from_be_bytes(NON_VALUE_BE) {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<I56> for Int56 {
    type Error = ();

    fn try_from(value: I56) -> Result<Self, Self::Error> {
        if value == I56::from_be_bytes(NON_VALUE_BE) {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<I56>> for Int56 {
    type Error = ();

    fn try_from(value: Option<I56>) -> Result<Self, Self::Error> {
        value.map_or_else(
            || Ok(Self(I56::from_be_bytes(NON_VALUE_BE))),
            Self::try_from,
        )
    }
}

impl TryFrom<i64> for Int56 {
    type Error = Option<i64>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        I56::try_from(value).map_or(Err(Some(value)), |i56| {
            Self::try_from(i56).map_err(|()| None)
        })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Int56 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Int56 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer)
            .map(I56::from_be_bytes)
            .map(Self)
    }
}
