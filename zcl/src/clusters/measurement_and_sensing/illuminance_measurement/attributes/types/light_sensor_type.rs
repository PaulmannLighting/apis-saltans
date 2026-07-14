use zb_core::types::{Enum8, Type, Uint8};

pub use self::manufacturer_specific::ManufacturerSpecific;

mod manufacturer_specific;

/// Type of light sensor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LightSensorType {
    /// The illuminance measurement is being performed by a photodiode.
    Photodiode,
    /// The illuminance measurement is being performed by a CMOS sensor.
    Cmos,
    /// Manufacturer-specific type of light sensor.
    ManufacturerSpecific(ManufacturerSpecific),
    /// The type of light sensor is unknown.
    Unknown,
}

impl LightSensorType {
    /// Return the raw value of the light sensor type.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Photodiode => 0x00,
            Self::Cmos => 0x01,
            Self::ManufacturerSpecific(manufacturer_specific) => manufacturer_specific.as_u8(),
            Self::Unknown => 0xff,
        }
    }
}

impl From<LightSensorType> for u8 {
    fn from(value: LightSensorType) -> Self {
        value.as_u8()
    }
}

impl From<LightSensorType> for Type {
    fn from(value: LightSensorType) -> Self {
        Self::Enum8(Enum8::new(Uint8::new(value.into())))
    }
}

impl TryFrom<u8> for LightSensorType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Photodiode),
            0x01 => Ok(Self::Cmos),
            ManufacturerSpecific::MIN..=ManufacturerSpecific::MAX => {
                ManufacturerSpecific::try_from(value).map(Self::ManufacturerSpecific)
            }
            0xff => Ok(Self::Unknown),
            other => Err(other),
        }
    }
}

impl TryFrom<Uint8> for LightSensorType {
    type Error = Uint8;

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|_| value)
    }
}

impl TryFrom<Type> for LightSensorType {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Enum8(value) = value {
            Self::try_from(value.into_inner()).map_err(|value| Type::Enum8(value.into()))
        } else {
            Err(value)
        }
    }
}
