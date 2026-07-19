use core::fmt::{self, Display};
use core::str::FromStr;

pub use self::application::{Application, ParseApplicationError};
pub use self::reserved::Reserved;

mod application;
mod reserved;

const DATA: u8 = 0x00;
const BROADCAST: u8 = 0xff;

/// A Zigbee endpoint ID.
///
/// Endpoints can be parsed from the exact `Data` or `Broadcast` variant name, a decimal endpoint
/// ID, or a hexadecimal endpoint ID with a `0x` prefix. Numeric application endpoint IDs produce
/// [`Endpoint::Application`]; reserved IDs are rejected.
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
            DATA => Ok(Self::Data),
            Application::MIN_ID..=Application::MAX_ID => Ok(Self::Application(Application(value))),
            Reserved::MIN_ID..=Reserved::MAX_ID => Err(Reserved(value)),
            BROADCAST => Ok(Self::Broadcast),
        }
    }

    /// Return the raw endpoint ID.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Data => DATA,
            Self::Application(application) => application.as_u8(),
            Self::Broadcast => BROADCAST,
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
            <Application as fmt::UpperHex>::fmt(&application, formatter)?;
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

impl FromStr for Endpoint {
    type Err = ParseEndpointError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Data" => Ok(Self::Data),
            "Broadcast" => Ok(Self::Broadcast),
            _ => Self::try_from(parse_endpoint_id(value)?).map_err(|_| ParseEndpointError),
        }
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

/// Error returned when parsing an unknown, malformed, or reserved Zigbee endpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseEndpointError;

impl Display for ParseEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid Zigbee endpoint")
    }
}

impl core::error::Error for ParseEndpointError {}

fn parse_endpoint_id(value: &str) -> Result<u8, ParseEndpointError> {
    value.strip_prefix("0x").map_or_else(
        || value.parse().map_err(|_| ParseEndpointError),
        |value| u8::from_str_radix(value, 16).map_err(|_| ParseEndpointError),
    )
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;

    use super::{Application, Endpoint, ParseEndpointError, Reserved};

    const APPLICATION_ID: u8 = 10;
    const APPLICATION_HEX: &str = "0x0A";

    #[test]
    fn parses_variant_names() {
        assert_eq!("Data".parse(), Ok(Endpoint::Data));
        assert_eq!("Broadcast".parse(), Ok(Endpoint::Broadcast));
    }

    #[test]
    fn parses_decimal_application_id() {
        assert_eq!(
            APPLICATION_ID.to_string().parse(),
            Ok(Endpoint::Application(
                Application::new(APPLICATION_ID).unwrap()
            ))
        );
    }

    #[test]
    fn parses_hexadecimal_application_id() {
        assert_eq!(
            APPLICATION_HEX.parse(),
            Ok(Endpoint::Application(
                Application::new(APPLICATION_ID).unwrap()
            ))
        );
    }

    #[test]
    fn parses_numeric_data_and_broadcast_ids() {
        assert_eq!("0".parse(), Ok(Endpoint::Data));
        assert_eq!("0xff".parse(), Ok(Endpoint::Broadcast));
    }

    #[test]
    fn rejects_reserved_id() {
        assert_eq!(
            Reserved::MIN_ID.to_string().parse::<Endpoint>(),
            Err(ParseEndpointError)
        );
    }

    #[test]
    fn rejects_unsupported_representations() {
        assert_eq!("Application".parse::<Endpoint>(), Err(ParseEndpointError));
        assert_eq!("Data (0x00)".parse::<Endpoint>(), Err(ParseEndpointError));
        assert_eq!("0X0A".parse::<Endpoint>(), Err(ParseEndpointError));
    }
}
