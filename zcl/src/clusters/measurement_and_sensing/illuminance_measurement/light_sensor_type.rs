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
            Self::Unknown => 0xFF,
        }
    }
}

impl From<LightSensorType> for u8 {
    fn from(value: LightSensorType) -> Self {
        value.as_u8()
    }
}

impl TryFrom<u8> for LightSensorType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Photodiode),
            0x01 => Ok(Self::Cmos),
            0x40..=0xFE => ManufacturerSpecific::try_from(value).map(Self::ManufacturerSpecific),
            0xFF => Ok(Self::Unknown),
            other => Err(other),
        }
    }
}
