use crate::{Endpoint, short_id};

/// Device destination with a short address and APS endpoint.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Device {
    device: short_id::Device,
    endpoint: Endpoint,
}

impl Device {
    /// Create a device destination from a short address and endpoint.
    #[must_use]
    pub const fn new(device: short_id::Device, endpoint: Endpoint) -> Self {
        Self { device, endpoint }
    }

    /// Return the destination device short address.
    #[must_use]
    pub const fn device(&self) -> short_id::Device {
        self.device
    }

    /// Return the destination APS endpoint.
    #[must_use]
    pub const fn endpoint(&self) -> Endpoint {
        self.endpoint
    }
}

impl_fmt_pair!(
    Device,
    short_id::Device,
    Endpoint,
    |value| (value.device, value.endpoint),
    ":"
);
