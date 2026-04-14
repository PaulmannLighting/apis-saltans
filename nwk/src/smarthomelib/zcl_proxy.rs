use std::time::Duration;

use bunt::Rgb;
use smarthomelib::{Action, ColorControl, Deciseconds, Executor, OnOff};
use zcl::Options;

use crate::Proxy;
use crate::proxies::endpoint::ZclProxy;

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
        crate::ColorControl::move_to_color(self, color, delay.deciseconds(), Options::default())
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
