pub use self::application::Application;
pub use self::broadcast::Broadcast;
pub use self::reserved::Reserved;

mod application;
mod broadcast;
mod reserved;

/// A Zigbee endpoint ID.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "u8", into = "u8")
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum Endpoint {
    /// Data interface of the Zigbee Device Object (ZDO).
    Data,

    /// Application-specific endpoint.
    Application(Application),

    /// Data interface broadcast endpoint.
    Broadcast,
}

impl Endpoint {
    /// Create a new `Endpoint` from a raw value.
    ///
    /// # Errors
    ///
    /// Returns [`Reserved`] when the raw value is in the reserved endpoint
    /// range.
    pub const fn try_new(value: u8) -> Result<Self, Reserved> {
        match value {
            0 => Ok(Self::Data),
            Application::MIN_ID..=Application::MAX_ID => Ok(Self::Application(Application(value))),
            Reserved::MIN_ID..=Reserved::MAX_ID => Err(Reserved(value)),
            255 => Ok(Self::Broadcast),
        }
    }

    /// Return the raw endpoint ID.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Data => 0,
            Self::Application(application) => application.as_u8(),
            Self::Broadcast => 255,
        }
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Application::default().into()
    }
}

impl_display_and_hex_via_value!(Endpoint, u8, |value| value.as_u8(), |value, formatter| {
    match *value {
        Self::Data => formatter.write_str("Data (0x00)"),
        Self::Application(application) => {
            formatter.write_str("Application (")?;
            <Application as core::fmt::UpperHex>::fmt(&application, formatter)?;
            formatter.write_str(")")
        }
        Self::Broadcast => formatter.write_str("Broadcast (0xff)"),
    }
});

impl From<Application> for Endpoint {
    fn from(application: Application) -> Self {
        Self::Application(application)
    }
}

impl From<Endpoint> for u8 {
    fn from(endpoint: Endpoint) -> Self {
        endpoint.as_u8()
    }
}

impl TryFrom<u8> for Endpoint {
    type Error = Reserved;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
