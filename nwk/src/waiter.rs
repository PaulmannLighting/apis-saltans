use tokio_mpmc::{ChannelError, Receiver};

use crate::Event;

/// Trait for checking the stack status.
pub trait Waiter {
    /// Wait for the expected event to occur.
    fn event(
        &mut self,
        expected_event: Event,
    ) -> impl Future<Output = Result<(), Option<ChannelError>>>;

    /// Wait for the network to be up.
    fn network_up(&mut self) -> impl Future<Output = Result<(), Option<ChannelError>>> {
        self.event(Event::NetworkUp)
    }

    /// Wait for the network to be down.
    fn network_down(&mut self) -> impl Future<Output = Result<(), Option<ChannelError>>> {
        self.event(Event::NetworkDown)
    }

    /// Wait for the network to be opened.
    fn network_opened(&mut self) -> impl Future<Output = Result<(), Option<ChannelError>>> {
        self.event(Event::NetworkOpened)
    }

    /// Wait for the network to be closed.
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
