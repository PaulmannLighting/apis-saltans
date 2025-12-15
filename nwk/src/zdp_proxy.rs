use le_stream::ToLeStream;
use zdp::Service;
use zigbee::Cluster;

use crate::frame::Type;
use crate::{Error, Frame, Proxy};

pub struct ZdpProxy<'proxy, T> {
    proxy: &'proxy T,
}

impl<'proxy, T> ZdpProxy<'proxy, T> {
    /// Create a new `ZclProxy`.
    pub(crate) const fn new(proxy: &'proxy T) -> Self {
        Self { proxy }
    }
}

impl<T> ZdpProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Send a ZCL command to a specific endpoint on a device.
    pub async fn unicast<C>(
        &self,
        pan_id: u16,
        endpoint: zigbee::Endpoint,
        command: C,
    ) -> Result<(), Error>
    where
        C: Cluster + Service + ToLeStream,
    {
        self.proxy
            .unicast(pan_id, endpoint, Frame::new(Type::Zdp(0x00), command))
            .await
    }
}
