use std::time::Duration;

use bunt::Rgb;
use smarthomelib::ColorControl;
use zcl::Options;

pub use self::error::Error;
use crate::Proxy;
use crate::proxies::endpoint::ZclProxy;

mod error;

const DECISECONDS_PER_MILLISECOND: u128 = 100;

impl<T> ColorControl for ZclProxy<'_, T>
where
    T: Proxy + Sync,
{
    type Error = Error;

    async fn move_to_color(&self, color: Rgb, delay: Duration) -> Result<(), Self::Error> {
        crate::ColorControl::move_to_color(
            self,
            color,
            (delay.as_millis() / DECISECONDS_PER_MILLISECOND).try_into()?,
            Options::default(),
        )
        .await?;
        Ok(())
    }
}
