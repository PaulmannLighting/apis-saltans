use le_stream::ToLeStream;
use zcl::Command;

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
    ) -> Result<(), Error>
    where
        C: Command + ToLeStream,
    {
        self.proxy
            .unicast(
                pan_id,
                endpoint,
                #[expect(unsafe_code)]
                // SAFETY: The sequence ID will be set by the receiving actor.
                unsafe { zcl::Frame::new_unsequenced(command) }.into(),
            )
            .await
    }
}
