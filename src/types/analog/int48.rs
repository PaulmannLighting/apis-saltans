use intx::I48;
use le_stream::derive::{FromLeStream, ToLeStream};

const NON_VALUE: [u8; 6] = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00];

/// The `48-bit signed integer` type, short `int48`.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream,
)]
#[repr(transparent)]
pub struct Int48(I48);

impl From<Int48> for Option<I48> {
    fn from(value: Int48) -> Self {
        if value.0 == I48::from_be_bytes(NON_VALUE) {
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
        if value == I48::from_be_bytes(NON_VALUE) {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<Option<I48>> for Int48 {
    type Error = ();

    fn try_from(value: Option<I48>) -> Result<Self, Self::Error> {
        value.map_or_else(|| Ok(Self(I48::from_be_bytes(NON_VALUE))), Self::try_from)
    }
}

impl TryFrom<i64> for Int48 {
    type Error = Option<i64>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        I48::try_from(value).map_or(Err(Some(value)), |i48| {
            Self::try_from(i48).map_err(|()| None)
        })
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Int48 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0.to_be_bytes())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Int48 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 6] = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self(I48::from_be_bytes(bytes)))
    }
}
