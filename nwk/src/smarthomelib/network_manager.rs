use smarthomelib::{Event, Receiver};

use super::Source;
use crate::{Coordinator, Ncp};

impl<T> Receiver<Source> for Coordinator<T>
where
    T: Ncp + Sync,
{
    async fn receive(&mut self) -> Option<Event<Source>> {
        self.recv().await.map(Into::into)
    }
}
