use le_stream::ToLeStream;
use zdp::{Frame, Service};
use zigbee::{Cluster, Endpoint};

use crate::{Error, Proxy};

/// A proxy structure to interact with ZDP commands on a specific device.
#[derive(Clone, Debug)]
pub struct ZdpProxy<'proxy, T> {
    proxy: &'proxy T,
    pan_id: u16,
}

impl<'proxy, T> ZdpProxy<'proxy, T> {
    /// Create a new `ZdpProxy`.
    pub(crate) const fn new(proxy: &'proxy T, pan_id: u16) -> Self {
        Self { proxy, pan_id }
    }
}

impl<T> ZdpProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Send a ZDP command to a specific endpoint on a device.
    pub async fn unicast<C>(&self, endpoint: Endpoint, command: C) -> Result<u8, Error>
    where
        C: Cluster + Service + ToLeStream,
    {
        self.proxy
            .unicast(
                self.pan_id,
                endpoint,
                Frame::new(self.proxy.next_transaction_seq().await?, command).into(),
            )
            .await
    }
}
