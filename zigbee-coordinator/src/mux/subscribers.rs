use tokio::sync::mpsc::Sender;
use zigbee_hw::Event;

#[derive(Debug, Default)]
pub struct Subscribers {
    subscribers: Vec<Sender<Event>>,
    retain: Vec<Sender<Event>>,
}

impl Subscribers {
    /// Send an event to all subscribers.
    pub async fn send(&mut self, event: &Event) {
        for subscriber in self.subscribers.drain(..) {
            // Only retain subscribes whose channels are not closed.
            if subscriber.send(event.clone()).await.is_ok() {
                self.retain.push(subscriber);
            }
        }

        self.subscribers.append(&mut self.retain);
    }

    /// Add a subscriber
    pub fn add(&mut self, subscriber: Sender<Event>) {
        self.subscribers.push(subscriber);
    }
}
