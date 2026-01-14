use le_stream::ToLeStream;
use zdp::Service;
use zigbee::{Cluster, Endpoint};

use crate::{Error, Proxy};

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
    pub async fn unicast<C>(&self, pan_id: u16, endpoint: Endpoint, command: C) -> Result<u8, Error>
    where
        C: Cluster + Service + ToLeStream,
    {
        self.proxy
            .unicast(
                pan_id,
                endpoint,
                zdp::Frame::new(self.proxy.next_transaction_seq().await, command).into(),
            )
            .await
    }
}
