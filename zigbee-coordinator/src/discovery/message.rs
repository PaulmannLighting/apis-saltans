use zigbee_hw::Event;

/// A message received by the discovery actor.
#[derive(Debug)]
pub enum Message {
    /// A hardware-level event.
    Event(Event),
    /// A signal to start discovery.
    StartDiscovery,
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self::Event(event)
    }
}
