use std::ops::{Deref, DerefMut};

use tokio::sync::mpsc::Receiver;

use crate::Event;

/// Newtype wrapper around a receiver of [`Event`]s.
#[derive(Debug)]
pub struct EventReceiver {
    inner: Receiver<Event>,
}

impl Deref for EventReceiver {
    type Target = Receiver<Event>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for EventReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<Receiver<Event>> for EventReceiver {
    fn from(inner: Receiver<Event>) -> Self {
        Self { inner }
    }
}
