use smarthomelib::{Event, Receiver};

use super::Source;
use crate::{NetworkManager, Proxy};

impl<T> Receiver<Source> for NetworkManager<T>
where
    T: Proxy + Sync,
{
    async fn receive(&mut self) -> Option<Event<Source>> {
        self.recv().await.map(Into::into)
    }
}
