use smarthomelib::{Action, Executor};

use crate::Proxy;
use crate::proxies::endpoint::ZclProxy;

impl<T> Executor for ZclProxy<'_, T>
where
    T: Proxy + Sync,
{
    type Error = crate::Error;

    async fn execute(&self, action: &Action) -> Result<(), Self::Error> {
        todo!("Implement")
    }
}
