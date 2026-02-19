use zcl::general::on_off::{Off, On, Toggle};

use crate::proxies::EndpointProxy;
use crate::{Error, Proxy};

/// Trait for On/Off cluster operations.
pub trait OnOff {
    /// Turns the device on.
    fn on(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Turns the device off.
    fn off(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Toggle the device state.
    fn toggle(&self) -> impl Future<Output = Result<u8, Error>> + Send;
}

impl<T> OnOff for EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    async fn on(&self) -> Result<u8, Error> {
        self.zcl().unicast(On).await
    }

    async fn off(&self) -> Result<u8, Error> {
        self.zcl().unicast(Off).await
    }

    async fn toggle(&self) -> Result<u8, Error> {
        self.zcl().unicast(Toggle).await
    }
}
