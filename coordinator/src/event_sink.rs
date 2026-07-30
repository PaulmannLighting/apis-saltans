use log::{debug, warn};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::TrySendError;

use crate::Event;

/// Non-blocking delivery handle for application-visible coordinator events.
#[derive(Clone, Debug)]
pub struct EventSink(Sender<Event>);

impl EventSink {
    /// Wrap the application-supplied event channel.
    #[must_use]
    pub const fn new(events: Sender<Event>) -> Self {
        Self(events)
    }

    /// Deliver an event without allowing application backpressure to stall protocol actors.
    ///
    /// Events are dropped when the application channel is full or closed.
    pub fn emit(&self, event: Event) {
        match self.0.try_send(event) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) => {
                warn!("Dropping coordinator event because the application event channel is full");
            }
            Err(TrySendError::Closed(_)) => {
                debug!(
                    "Dropping coordinator event because the application event channel is closed"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::channel;

    use super::EventSink;
    use crate::{Event, Network};

    const CHANNEL_SIZE: usize = 1;

    #[test]
    fn full_application_channel_drops_the_new_event() {
        let (events, mut received) = channel(CHANNEL_SIZE);
        let sink = EventSink::new(events);
        sink.emit(Event::Network(Network::Up));

        sink.emit(Event::Network(Network::Down));

        assert!(matches!(
            received.try_recv(),
            Ok(Event::Network(Network::Up))
        ));
        assert!(received.try_recv().is_err());
    }
}
