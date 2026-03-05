use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;

pub use self::information::Information;
pub use self::settings::Settings;

mod information;
mod settings;

const MASK: u16 = 0x00f0;
const MODULO: u16 = 0x0020;
const INFORMATION: u16 = 0x0000;
const SETTINGS: u16 = 0x0010;

/// Attributes related to battery information and settings.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant)]
pub enum Battery {
    /// Information about the battery status of a device.
    Information(Information) = INFORMATION,
    /// Settings related to the battery configuration.
    Settings(Settings) = SETTINGS,
}

impl Battery {
    /// Returns the attribute mask.
    #[must_use]
    pub const fn mask(&self) -> u16 {
        match self {
            Self::Information(info) => self.discriminant() | info.discriminant(),
            Self::Settings(settings) => self.discriminant() | settings.discriminant(),
        }
    }
}

impl FromLeStreamTagged for Battery {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match (tag & MASK) % MODULO {
            INFORMATION => {
                Ok(Information::from_le_stream_tagged(tag, bytes)?.map(Self::Information))
            }
            SETTINGS => Ok(Settings::from_le_stream_tagged(tag, bytes)?.map(Self::Settings)),
            _ => Err(tag),
        }
    }
}
