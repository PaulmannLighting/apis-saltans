use intx::I40;
use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE_BE: [u8; 5] = [0x80, 0x00, 0x00, 0x00, 0x00];

/// The `40-bit signed integer` type, short `int40`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int40(I40);

impl From<Int40> for Option<I40> {
    fn from(value: Int40) -> Self {
        value.try_into().ok()
    }
}

impl From<Int40> for Option<i64> {
    fn from(value: Int40) -> Self {
        Option::<I40>::from(value).map(Into::into)
    }
}

impl TryFrom<Int40> for I40 {
    type Error = ();

    fn try_from(value: Int40) -> Result<Self, Self::Error> {
        if value.0 == Self::from_be_bytes(NON_VALUE_BE) {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl TryFrom<I40> for Int40 {
    type Error = ();

    fn try_from(value: I40) -> Result<Self, Self::Error> {
        if value == I40::from_be_bytes(NON_VALUE_BE) {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<I40>> for Int40 {
    type Error = ();

    fn try_from(value: Option<I40>) -> Result<Self, Self::Error> {
        value.map_or_else(
            || Ok(Self(I40::from_be_bytes(NON_VALUE_BE))),
            Self::try_from,
        )
    }
}

impl TryFrom<i64> for Int40 {
    type Error = Option<i64>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        I40::try_from(value).map_or(Err(Some(value)), |i40| {
            Self::try_from(i40).map_err(|()| None)
        })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Int40 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Int40 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer)
            .map(I40::from_be_bytes)
            .map(Self)
    }
}
