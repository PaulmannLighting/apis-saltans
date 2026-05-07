use smarthomelib::OnOff;

use crate::Ncp;
use crate::proxies::endpoint::ZclProxy;

impl<T> OnOff for ZclProxy<'_, T>
where
    T: Ncp + Sync,
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
