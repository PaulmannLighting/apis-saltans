use intx::I24;
use le_stream::{FromLeStream, ToLeStream};

const NON_VALUE_BE: [u8; 3] = [0x80, 0x00, 0x00];

/// The `24-bit signed integer` type, short `int24`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int24(I24);

impl From<Int24> for Option<I24> {
    fn from(value: Int24) -> Self {
        value.try_into().ok()
    }
}

impl TryFrom<Int24> for I24 {
    type Error = ();

    fn try_from(value: Int24) -> Result<Self, Self::Error> {
        if value.0 == Self::from_be_bytes(NON_VALUE_BE) {
            Err(())
        } else {
            Ok(value.0)
        }
    }
}

impl From<Int24> for Option<i32> {
    fn from(value: Int24) -> Self {
        Option::<I24>::from(value).map(Into::into)
    }
}

impl TryFrom<I24> for Int24 {
    type Error = ();

    fn try_from(value: I24) -> Result<Self, Self::Error> {
        if value == I24::from_be_bytes(NON_VALUE_BE) {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<I24>> for Int24 {
    type Error = ();

    fn try_from(value: Option<I24>) -> Result<Self, Self::Error> {
        value.map_or_else(
            || Ok(Self(I24::from_be_bytes(NON_VALUE_BE))),
            Self::try_from,
        )
    }
}

impl TryFrom<i32> for Int24 {
    type Error = Option<i32>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        I24::try_from(value).map_or(Err(Some(value)), |i24| {
            Self::try_from(i24).map_err(|()| None)
        })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Int24 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_be_bytes().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Int24 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer)
            .map(I24::from_be_bytes)
            .map(Self)
    }
}
