use std::ops::Deref;

use macaddr::MacAddr8;
use zigbee::Endpoint;

use self::zcl::ZclProxy;
use self::zdp::ZdpProxy;
use crate::{Error, Frame, Proxy};

mod zcl;
mod zdp;

/// A proxy structure to interact with a specific endpoint on a Zigbee device.
#[derive(Clone, Debug)]
pub struct EndpointProxy<'proxy, T> {
    proxy: &'proxy T,
    pan_id: u16,
    endpoint: Endpoint,
}

impl<'proxy, T> EndpointProxy<'proxy, T> {
    /// Create a new `EndpointProxy`.
    pub(crate) const fn new(proxy: &'proxy T, pan_id: u16, endpoint: Endpoint) -> Self {
        Self {
            proxy,
            pan_id,
            endpoint,
        }
    }

    /// Return a ZCL proxy for the endpoint.
    #[must_use]
    pub const fn zcl(&self) -> ZclProxy<'proxy, T> {
        ZclProxy::new(self.proxy, self.pan_id, self.endpoint)
    }

    /// Return a ZDP proxy for the endpoint.
    #[must_use]
    pub const fn zdp(&self) -> ZdpProxy<'proxy, T> {
        ZdpProxy::new(self.proxy, self.pan_id, self.endpoint)
    }
}

impl<T> EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Get the PAN ID of the device.
    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }

    /// Get the endpoint of the device.
    #[must_use]
    pub const fn endpoint(&self) -> Endpoint {
        self.endpoint
    }

    /// Get the IEEE address of the device.
    ///
    /// TODO: Cache the result to avoid multiple requests.
    pub async fn ieee_address(&self) -> Result<MacAddr8, Error> {
        self.proxy.get_ieee_address(self.pan_id).await
    }

    /// Send a frame to a specific endpoint on the device.
    ///
    /// # Errors
    ///
    /// This function will return an error if the frame could not be sent.
    pub async fn unicast(&self, frame: Frame) -> Result<u8, Error> {
        self.proxy.unicast(self.pan_id, self.endpoint, frame).await
    }
}

impl<T> Deref for EndpointProxy<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.proxy
    }
}
