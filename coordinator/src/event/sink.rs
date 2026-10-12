use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use log::{debug, warn};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::TrySendError;

use super::Event;

const EVENT_COUNT_STEP: u64 = 1;

/// Non-blocking delivery handle for application-visible coordinator events.
#[derive(Clone, Debug)]
pub struct EventSink {
    sender: Sender<Event>,
    losses: Arc<LossCounters>,
}

/// Application events lost to channel backpressure or closure.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DroppedEvents {
    /// Events discarded because the receiver was not keeping up.
    pub full: u64,
    /// Events discarded after the receiver was dropped.
    pub closed: u64,
}

#[derive(Debug, Default)]
struct LossCounters {
    full: AtomicU64,
    closed: AtomicU64,
}

impl EventSink {
    /// Wrap the application-supplied event channel.
    #[must_use]
    pub fn new(events: Sender<Event>) -> Self {
        Self {
            sender: events,
            losses: Arc::default(),
        }
    }

    /// Read cumulative losses independently of the event channel.
    pub fn dropped_events(&self) -> DroppedEvents {
        DroppedEvents {
            full: self.losses.full.load(Ordering::Relaxed),
            closed: self.losses.closed.load(Ordering::Relaxed),
        }
    }

    /// Deliver an event without allowing application backpressure to stall protocol actors.
    ///
    /// Events are dropped when the application channel is full or closed.
    pub fn emit(&self, event: Event) {
        match self.sender.try_send(event) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) => {
                self.losses
                    .full
                    .fetch_add(EVENT_COUNT_STEP, Ordering::Relaxed);
                warn!("Dropping coordinator event because the application event channel is full");
            }
            Err(TrySendError::Closed(_)) => {
                self.losses
                    .closed
                    .fetch_add(EVENT_COUNT_STEP, Ordering::Relaxed);
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
    use crate::event::{Event, Network};

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
