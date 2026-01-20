use tokio::sync::mpsc::Receiver;

use crate::Event;

/// Trait for waiting on Zigbee network events.
pub trait Waiter {
    /// Wait for the expected event to occur.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the underlying channel is closed while still waiting for the event.
    fn event(&mut self, expected_event: Event) -> impl Future<Output = Result<(), ()>> + Send;

    /// Wait for the network to be up.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the underlying channel is closed while still waiting for the event.
    fn network_up(&mut self) -> impl Future<Output = Result<(), ()>> + Send {
        self.event(Event::NetworkUp)
    }

    /// Wait for the network to be down.
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the underlying channel is closed while still waiting for the event.
    fn network_down(&mut self) -> impl Future<Output = Result<(), ()>> + Send {
        self.event(Event::NetworkDown)
    }

    /// Wait for the network to be opened for new nodes to join ("_joinable_").
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the underlying channel is closed while still waiting for the event.
    fn network_opened(&mut self) -> impl Future<Output = Result<(), ()>> + Send {
        self.event(Event::NetworkOpened)
    }

    /// Wait for the network to be closed for new nodes to join ("_not joinable_").
    ///
    /// # Errors
    ///
    /// Returns `Err(())` if the underlying channel is closed while still waiting for the event.
    fn network_closed(&mut self) -> impl Future<Output = Result<(), ()>> + Send {
        self.event(Event::NetworkClosed)
    }
}

impl Waiter for Receiver<Event> {
    async fn event(&mut self, expected_event: Event) -> Result<(), ()> {
        while let Some(received_event) = self.recv().await {
            if received_event == expected_event {
                return Ok(());
            }
        }

        Err(())
    }
}
