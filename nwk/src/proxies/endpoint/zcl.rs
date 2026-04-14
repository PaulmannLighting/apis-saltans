use le_stream::ToLeStream;
use zcl::HeaderFactory;
use zigbee::{ClusterId, Endpoint};

use crate::{Error, Proxy};

/// A proxy structure to interact with ZCL commands on a specific endpoint.
#[derive(Clone, Debug)]
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
    pub async fn unicast<P>(&self, payload: P) -> Result<u8, Error>
    where
        P: HeaderFactory + ClusterId + ToLeStream,
    {
        self.proxy
            .unicast(
                self.pan_id,
                self.endpoint,
                payload
                    .frame(self.proxy.next_transaction_seq().await?)
                    .into(),
            )
            .await
    }
}

#[cfg(feature = "smarthomelib")]
mod smarthomelib {
    use std::time::Duration;

    use bunt::Rgb;
    use smarthomelib::{Action, ColorControl, Deciseconds, Executor, OnOff};
    use zcl::Options;

    use super::ZclProxy;
    use crate::Proxy;

    impl<T> OnOff for ZclProxy<'_, T>
    where
        T: Proxy + Sync,
    {
        type Error = crate::Error;

        async fn on(&self) -> Result<(), Self::Error> {
            crate::zcl::OnOff::on(self).await?;
            Ok(())
        }

        async fn off(&self) -> Result<(), Self::Error> {
            crate::zcl::OnOff::off(self).await?;
            Ok(())
        }

        async fn toggle(&self) -> Result<(), Self::Error> {
            crate::zcl::OnOff::toggle(self).await?;
            Ok(())
        }
    }

    impl<T> ColorControl for ZclProxy<'_, T>
    where
        T: Proxy + Sync,
    {
        type Error = crate::Error;

        async fn move_to_color(&self, color: Rgb, delay: Duration) -> Result<(), Self::Error> {
            crate::ColorControl::move_to_color(
                self,
                color,
                delay.deciseconds(),
                Options::default(),
            )
            .await?;
            Ok(())
        }
    }

    impl<T> Executor for ZclProxy<'_, T>
    where
        T: Proxy + Sync,
    {
        type Error = crate::Error;

        async fn execute(&self, action: &Action) -> Result<(), Self::Error> {
            todo!("Implement")
        }
    }
}
