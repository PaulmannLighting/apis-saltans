//! ZCL transmission layer.

use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::Receiver;
use zigbee::Endpoint;

pub use self::transceiver::Transceiver;
pub use self::transmitter::Transmitter;
use crate::demux::{Message, Subscribe};
use crate::{DeviceProxy, Error, Event, Frame};

mod receiver;
mod transceiver;
mod transmitter;

/// ZCL transceiver struct.
// TODO: Find a better name.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ZclTransceiver<T, R> {
    transmitter: T,
    receiver: R,
}

impl<T, R> ZclTransceiver<T, R> {
    /// Crate a new ZCL transceiver.
    #[must_use]
    pub const fn new(transmitter: T, receiver: R) -> Self {
        Self {
            transmitter,
            receiver,
        }
    }

    /// Return a device proxy.
    pub fn device(self, pan_id: u16) -> DeviceProxy<Self> {
        DeviceProxy::new(self, pan_id)
    }
}

impl<T, R> Transmitter for ZclTransceiver<T, R>
where
    T: Transmitter,
{
    fn next_seq(&self) -> impl Future<Output = Result<u8, Error>> + Send {
        self.transmitter.next_seq()
    }

    fn send<F>(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        frame: F,
    ) -> impl Future<Output = Result<u8, Error>> + Send
    where
        F: Into<Frame> + Send,
    {
        self.transmitter.send(pan_id, endpoint, frame)
    }
}

impl<T, R> Subscribe for ZclTransceiver<T, R>
where
    R: Subscribe,
{
    fn subscribe(
        &self,
        seq: u8,
    ) -> impl Future<Output = Result<Receiver<Event>, SendError<Message>>> + Send {
        self.receiver.subscribe(seq)
    }
}
