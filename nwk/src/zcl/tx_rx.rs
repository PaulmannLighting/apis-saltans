//! ZCL transmission layer.

use zcl::{Cluster, Frame, HeaderFactory};

use crate::Error;

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

/// ZCL transmission and reception layer.
pub trait TxRx {
    /// Communicate two-way. Send and receive a ZCL frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending or receiving the frame fails.
    fn communicate<R, C>(&self, frame: C) -> impl Future<Output = Result<R, Error>> + Send
    where
        C: HeaderFactory + Send;
}

impl<T> TxRx for T
where
    T: Tx + Rx + Sync,
{
    async fn communicate<R, C>(&self, frame: C) -> Result<R, Error>
    where
        C: HeaderFactory + Send,
    {
        let seq = self.next_seq().await;
        self.send(seq, frame).await?;
        let response = self.recv(seq).await?;
        todo!("Use a macro or so to parse `R` from the response frame.")
    }
}
