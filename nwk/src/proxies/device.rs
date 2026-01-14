use std::ops::Deref;

use macaddr::MacAddr8;
use zigbee::Endpoint;

use self::zcl::ZclProxy;
use self::zdp::ZdpProxy;
use super::endpoint::EndpointProxy;
use crate::{Error, Frame, Proxy};

mod zcl;
mod zdp;

/// A proxy structure to interact with a Zigbee device.
#[derive(Clone, Debug)]
pub struct DeviceProxy<'proxy, T> {
    proxy: &'proxy T,
    pan_id: u16,
}

impl<'proxy, T> DeviceProxy<'proxy, T> {
    /// Create a new `DeviceProxy`.
    pub(crate) const fn new(proxy: &'proxy T, pan_id: u16) -> Self {
        Self { proxy, pan_id }
    }

    /// Return a ZCL proxy for the device.
    #[must_use]
    pub const fn zcl(&self) -> ZclProxy<'proxy, T> {
        ZclProxy::new(self.proxy, self.pan_id)
    }

    /// Return a ZDP proxy for the device.
    #[must_use]
    pub const fn zdp(&self) -> ZdpProxy<'proxy, T> {
        ZdpProxy::new(self.proxy, self.pan_id)
    }

    /// Get a proxy for a specific endpoint on the device.
    pub const fn endpoint(&self, endpoint_id: Endpoint) -> EndpointProxy<'proxy, T> {
        EndpointProxy::new(self.proxy, self.pan_id, endpoint_id)
    }

    /// Get a proxy for the default endpoint on the device.
    pub fn default_endpoint(&self) -> EndpointProxy<'proxy, T> {
        self.endpoint(Endpoint::default())
    }
}

impl<T> DeviceProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Get the PAN ID of the device.
    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
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
    pub async fn unicast(&self, endpoint: Endpoint, frame: Frame) -> Result<u8, Error> {
        self.proxy.unicast(self.pan_id, endpoint, frame).await
    }
}

impl<T> Deref for DeviceProxy<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.proxy
    }
}
