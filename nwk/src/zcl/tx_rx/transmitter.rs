use zcl::HeaderFactory;
use zigbee::Endpoint;

use crate::{Error, Frame, Proxy};

/// ZCL transmission layer.
pub trait Transmitter {
    /// Return the next sequence number.
    ///
    /// # Error
    ///
    /// Returns an [`Error`] if obtaining the sequence number fails.
    fn next_seq(&self) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Send a ZCL frame.
    ///
    /// # Error
    ///
    /// Returns an [`Error`] if sending the frame fails.
    fn send<T>(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        frame: T,
    ) -> impl Future<Output = Result<u8, Error>> + Send
    where
        T: Into<Frame> + Send;
}

impl<T> Transmitter for T
where
    T: Proxy + Sync,
{
    async fn next_seq(&self) -> Result<u8, Error> {
        Proxy::next_transaction_seq(self).await
    }

    async fn send<F>(&self, pan_id: u16, endpoint: Endpoint, frame: F) -> Result<u8, Error>
    where
        F: Into<Frame> + Send,
    {
        Proxy::unicast(self, pan_id, endpoint, frame.into()).await
    }
}
