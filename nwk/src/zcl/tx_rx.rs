//! ZCL transmission layer.

use zcl::{Cluster, Frame, HeaderFactory};

use crate::{Command, DemuxProxy, Error, Event};

/// ZCL transmission layer.
pub trait Tx {
    /// Return the next sequence number.
    fn next_seq(&self) -> impl Future<Output = u8> + Send;

    /// Send a ZCL frame.
    ///
    /// # Error
    ///
    /// Returns an [`Error`] if sending the frame fails.
    fn send<T>(&self, seq: u8, frame: T) -> impl Future<Output = Result<(), Error>> + Send
    where
        T: HeaderFactory;
}

/// ZCL reception layer.
pub trait Rx {
    /// Receive a ZCL frame.
    ///
    /// # Error
    ///
    /// Returns an [`Error`] if receiving the frame fails.
    fn recv(&self, seq: u8) -> impl Future<Output = Result<Frame<Cluster>, Error>> + Send;
}

// TODO: Remove panicking paths.
impl<T> Rx for T
where
    T: DemuxProxy + Sync,
{
    async fn recv(&self, seq: u8) -> Result<Frame<Cluster>, Error> {
        let event = self.recv(seq).await?;

        let (src_address, (aps_header, command)) = match event {
            Event::MessageReceived {
                src_address,
                aps_frame,
            } => (src_address, aps_frame.into_parts()),
            other => todo!("Handle unexpected event."),
        };

        match command {
            Command::Zcl(frame) => Ok(frame),
            other => todo!("Handle unexpected command type."),
        }
    }
}

/// ZCL transmission and reception layer.
pub trait TxRx {
    /// Communicate two-way. Send and receive a ZCL frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending or receiving the frame fails.
    fn communicate<T>(
        &self,
        frame: T,
    ) -> impl Future<Output = Result<Frame<Cluster>, Error>> + Send
    where
        T: HeaderFactory + Send;
}

impl<T> TxRx for T
where
    T: Tx + Rx + Sync,
{
    async fn communicate<U>(&self, frame: U) -> Result<Frame<Cluster>, Error>
    where
        U: HeaderFactory + Send,
    {
        let seq = self.next_seq().await;
        self.send(seq, frame).await?;
        self.recv(seq).await
    }
}
