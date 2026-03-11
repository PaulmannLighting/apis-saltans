use le_stream::ToLeStream;
use zcl::HeaderFactory;
use zigbee::{ClusterId, Endpoint};

use crate::{Error, Proxy};

/// A proxy structure to send ZCL commands.
#[derive(Clone, Copy, Debug)]
pub struct ZclProxy<'proxy, T> {
    proxy: &'proxy T,
}

impl<'proxy, T> ZclProxy<'proxy, T> {
    /// Create a new `ZclProxy`.
    pub(crate) const fn new(proxy: &'proxy T) -> Self {
        Self { proxy }
    }
}

impl<T> ZclProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Send a ZCL command to a specific endpoint on a device.
    pub async fn unicast<P>(&self, pan_id: u16, endpoint: Endpoint, payload: P) -> Result<u8, Error>
    where
        P: HeaderFactory + ClusterId + ToLeStream,
    {
        self.proxy
            .unicast(
                pan_id,
                endpoint,
                payload
                    .frame(self.proxy.next_transaction_seq().await?)
                    .into(),
            )
            .await
    }
}
