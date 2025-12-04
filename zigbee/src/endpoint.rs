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

impl Default for Endpoint {
    fn default() -> Self {
        DEFAULT_ENDPOINT.into()
    }
}

impl From<u8> for Endpoint {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Data,
            Application::MIN..=Application::MAX => Self::Application(
                #[expect(unsafe_code)]
                // SAFETY: We just checked, that the alue is within `1..=240`.
                unsafe {
                    Application::new_unchecked(value)
                },
            ),
            Reserved::MIN..=Reserved::MAX => Self::Reserved(
                #[expect(unsafe_code)]
                // SAFETY: We just checked, that the alue is within `241..=254`.
                unsafe {
                    Reserved::new_unchecked(value)
                },
            ),
            255 => Self::Broadcast,
        }
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
