use tokio_mpmc::{ChannelError, Receiver};

use crate::Event;

/// Trait for waiting on Zigbee network events.
pub trait Waiter {
    /// Wait for the expected event to occur.
    ///
    /// # Errors
    ///
    /// - `Some(ChannelError)` if the underlying channel is closed while there were still messaged queued.
    /// - `None` if the underlying channel closed without any left messages queued.
    fn event(
        &mut self,
        expected_event: Event,
    ) -> impl Future<Output = Result<(), Option<ChannelError>>>;

    /// Wait for the network to be up.
    ///
    /// # Errors
    ///
    /// - `Some(ChannelError)` if the underlying channel is closed while there were still messaged queued.
    /// - `None` if the underlying channel closed without any left messages queued.
    fn network_up(&mut self) -> impl Future<Output = Result<(), Option<ChannelError>>> {
        self.event(Event::NetworkUp)
    }

    /// Wait for the network to be down.
    ///
    /// # Errors
    ///
    /// - `Some(ChannelError)` if the underlying channel is closed while there were still messaged queued.
    /// - `None` if the underlying channel closed without any left messages queued.
    fn network_down(&mut self) -> impl Future<Output = Result<(), Option<ChannelError>>> {
        self.event(Event::NetworkDown)
    }

    /// Wait for the network to be opened for new nodes to join ("_joinable_").
    ///
    /// # Errors
    ///
    /// - `Some(ChannelError)` if the underlying channel is closed while there were still messaged queued.
    /// - `None` if the underlying channel closed without any left messages queued.
    fn network_opened(&mut self) -> impl Future<Output = Result<(), Option<ChannelError>>> {
        self.event(Event::NetworkOpened)
    }

    /// Wait for the network to be closed for new nodes to join ("_not joinable_").
    ///
    /// # Errors
    ///
    /// - `Some(ChannelError)` if the underlying channel is closed while there were still messaged queued.
    /// - `None` if the underlying channel closed without any left messages queued.
    fn network_closed(&mut self) -> impl Future<Output = Result<(), Option<ChannelError>>> {
        self.event(Event::NetworkClosed)
    }
}

impl Waiter for Receiver<Event> {
    async fn event(&mut self, expected_event: Event) -> Result<(), Option<ChannelError>> {
        while let Some(received_event) = self.recv().await? {
            if received_event == expected_event {
                return Ok(());
            }
        }

        Err(None)
    }
}
