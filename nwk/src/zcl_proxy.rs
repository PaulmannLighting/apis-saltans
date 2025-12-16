use le_stream::ToLeStream;
use zcl::Command;
use zigbee::ClusterId;

use crate::{Error, Proxy};

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
    pub async fn unicast<C>(
        &self,
        pan_id: u16,
        endpoint: zigbee::Endpoint,
        command: C,
    ) -> Result<u8, Error>
    where
        C: Command + ClusterId + ToLeStream,
    {
        let seq = self.proxy.next_transaction_seq().await;
        self.proxy
            .unicast(pan_id, endpoint, zcl::Frame::new(seq, command).into())
            .await
    }
}
