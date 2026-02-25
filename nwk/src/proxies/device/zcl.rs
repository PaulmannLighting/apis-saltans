use le_stream::ToLeStream;
use zcl::{Command, Frame};
use zigbee::{ClusterId, Endpoint};

use crate::{Error, Proxy};

/// A proxy structure to interact with ZCL commands on a specific device.
#[derive(Clone, Debug)]
pub struct ZclProxy<'proxy, T> {
    proxy: &'proxy T,
    pan_id: u16,
}

impl<'proxy, T> ZclProxy<'proxy, T> {
    /// Create a new `ZclProxy`.
    pub(crate) const fn new(proxy: &'proxy T, pan_id: u16) -> Self {
        Self { proxy, pan_id }
    }
}

impl<T> ZclProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Send a ZCL command to a specific endpoint on a device.
    pub async fn unicast<C>(&self, endpoint: Endpoint, command: C) -> Result<u8, Error>
    where
        C: Command + ClusterId + ToLeStream,
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
