use le_stream::ToLeStream;
use zcl::{Cluster, Frame, HeaderFactory};
use zigbee::Endpoint;

use crate::Error;
use crate::demux::Subscribe;
use crate::zcl::tx_rx::receiver::Receiver;
use crate::zcl::tx_rx::transmitter::Transmitter;

/// ZCL transmission and reception layer.
pub trait Transceiver {
    /// Communicate two-way. Send and receive a ZCL frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending or receiving the frame fails.
    fn communicate<T>(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        frame: T,
    ) -> impl Future<Output = Result<Frame<Cluster>, Error>> + Send
    where
        T: zigbee::Cluster + HeaderFactory + ToLeStream + Send;
}

impl<T> Transceiver for T
where
    T: Transmitter + Subscribe + Sync,
{
    async fn communicate<F>(
        &self,
        short_id: u16,
        endpoint: Endpoint,
        frame: F,
    ) -> Result<Frame<Cluster>, Error>
    where
        F: zigbee::Cluster + HeaderFactory + ToLeStream + Send,
    {
        let seq = self.next_seq().await?;
        let response = self.subscribe(seq).await?;
        self.send(short_id, endpoint, frame.frame(seq)).await?;
        response.recv().await
    }
}
