use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::{Type, Uint16};

const MAX: u16 = 0x00ff;

/// Alarm count type.
///
/// # Invariants
///
/// This type guarantees that the value is in the range `0x00` to `0xff`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord, FromLeStream, ToLeStream,
)]
pub struct AlarmCount(Uint16);

impl AlarmCount {
    /// Create a new `AlarmCount`.
    #[must_use]
    pub const fn new(value: Option<u8>) -> Self {
        match value {
            Some(value) => Self(Uint16::new(value as u16)),
            None => Self(Uint16::NONE),
        }
    }
}

impl From<u8> for AlarmCount {
    fn from(value: u8) -> Self {
        Self(Uint16::new(value.into()))
    }
}

impl From<Option<u8>> for AlarmCount {
    fn from(value: Option<u8>) -> Self {
        Self::new(value)
    }
}

impl From<AlarmCount> for Uint16 {
    fn from(value: AlarmCount) -> Self {
        value.0
    }
}

impl From<AlarmCount> for Type {
    fn from(value: AlarmCount) -> Self {
        value.0.into()
    }
}

impl From<AlarmCount> for Option<u8> {
    fn from(value: AlarmCount) -> Self {
        if value.0 == Uint16::NONE {
            None
        } else {
            #[expect(clippy::cast_possible_truncation)]
            // We guarantee that the value is in the range `0x00` to `0xff`.
            Some(value.0.as_u16() as u8)
        }
    }
}

impl TryFrom<Uint16> for AlarmCount {
    type Error = Uint16;

    fn try_from(value: Uint16) -> Result<Self, Self::Error> {
        if value.as_u16() <= MAX || value == Uint16::NONE {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}

impl TryFrom<Type> for AlarmCount {
    type Error = Type;

    fn try_from(typ: Type) -> Result<Self, Self::Error> {
        typ.try_into().map(Self)
    }
}
