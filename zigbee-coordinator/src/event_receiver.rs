use std::ops::{Deref, DerefMut};

use tokio::sync::mpsc::Receiver;

use crate::Event;

/// Newtype wrapper around a receiver of [`Event`]s.
#[derive(Debug)]
pub struct EventReceiver {
    pub(crate) inner: Receiver<Event>,
}

impl From<Receiver<Event>> for EventReceiver {
    fn from(inner: Receiver<Event>) -> Self {
        Self { inner }
    }
}
