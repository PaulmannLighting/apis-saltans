use std::ops::{Deref, DerefMut};

use tokio::sync::mpsc::Receiver;

use crate::Event;

/// A receiver for coordinator [`Event`]s.
///
/// This wrapper dereferences to Tokio's [`Receiver<Event>`], so methods such as
/// [`Receiver::recv`] are available directly.
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
