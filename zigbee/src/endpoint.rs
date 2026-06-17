use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};

pub use self::application::Application;
pub use self::reserved::Reserved;

mod application;
mod reserved;

const DEFAULT_ENDPOINT: u8 = 0x01;

/// A Zigbee endpoint ID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum Endpoint {
    /// Data interface of the Zigbee Device Object (ZDO).
    Data,

    /// Application-specific endpoint.
    Application(Application),

    /// Reserved endpoint.
    Reserved(Reserved),

    /// Data interface broadcast endpoint.
    Broadcast,
}

impl Endpoint {
    /// Create a new `Endpoint` from a raw value.
    #[must_use]
    pub const fn new(value: u8) -> Self {
        match value {
            0 => Self::Data,
            Application::MIN..=Application::MAX => Self::Application(Application(value)),
            Reserved::MIN..=Reserved::MAX => Self::Reserved(Reserved(value)),
            255 => Self::Broadcast,
        }
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        DEFAULT_ENDPOINT.into()
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data => write!(f, "Data (0x00)"),
            Self::Application(app) => write!(f, "Application ({:#04X})", u8::from(*app)),
            Self::Reserved(res) => write!(f, "Reserved ({:#04X})", u8::from(*res)),
            Self::Broadcast => write!(f, "Broadcast (0xff)"),
        }
    }
}

impl From<u8> for Endpoint {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<Application> for Endpoint {
    fn from(application: Application) -> Self {
        Self::Application(application)
    }
}

impl From<Endpoint> for u8 {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint {
            Endpoint::Data => 0,
            Endpoint::Application(application) => application.into(),
            Endpoint::Reserved(reserved) => reserved.into(),
            Endpoint::Broadcast => 255,
        }
    }
}

impl FromLeStream for Endpoint {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u8::from_le_stream(bytes).map(Into::into)
    }
}

impl ToLeStream for Endpoint {
    type Iter = <u8 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        u8::from(self).to_le_stream()
    }
}
