use std::collections::BTreeMap;

use tokio::sync::oneshot::Sender;

use crate::Event;

/// A map of subscribers.
pub type Subscribers = BTreeMap<u8, Sender<Event>>;
