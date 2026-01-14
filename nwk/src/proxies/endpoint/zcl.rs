use std::ops::Deref;

use le_stream::ToLeStream;
use zcl::{Command, Frame};
use zigbee::{ClusterId, Endpoint};

use crate::{Error, Proxy};

/// A proxy structure to interact with ZCL commands on a specific endpoint.
#[derive(Clone, Copy, Debug)]
pub struct ZclProxy<'proxy, T> {
    proxy: &'proxy T,
    pan_id: u16,
    endpoint: Endpoint,
}

impl<'proxy, T> ZclProxy<'proxy, T> {
    /// Create a new `ZclProxy`.
    pub(crate) const fn new(proxy: &'proxy T, pan_id: u16, endpoint: Endpoint) -> Self {
        Self {
            proxy,
            pan_id,
            endpoint,
        }
    }
}

impl<T> ZclProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Send a ZCL command to a specific endpoint on a device.
    pub async fn unicast<C>(&self, command: C) -> Result<u8, Error>
    where
        C: Command + ClusterId + ToLeStream,
    {
        self.proxy
            .unicast(
                self.pan_id,
                self.endpoint,
                Frame::new(self.next_transaction_seq().await, command).into(),
            )
            .await
    }
}

impl<T> Deref for ZclProxy<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.proxy
    }
}
