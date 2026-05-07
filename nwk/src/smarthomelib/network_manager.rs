use smarthomelib::{Event, Receiver};

use super::Source;
use crate::{Ncp, NetworkManager};

impl<T> Receiver<Source> for NetworkManager<T>
where
    T: Ncp + Sync,
{
    async fn receive(&mut self) -> Option<Event<Source>> {
        self.recv().await.map(Into::into)
    }
}
