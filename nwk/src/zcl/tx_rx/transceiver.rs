use le_stream::ToLeStream;
use zcl::{Cluster, Frame, HeaderFactory};
use zigbee::Endpoint;

use crate::zcl::tx_rx::receiver::Receiver;
use crate::zcl::tx_rx::transmitter::Transmitter;
use crate::{DemuxProxy, Error};

/// ZCL transmission and reception layer.
pub trait Transceiver {
    /// Communicate two-way. Send and receive a ZCL frame.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if sending or receiving the frame fails.
    fn communicate<T>(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        frame: T,
    ) -> impl Future<Output = Result<Frame<Cluster>, Error>> + Send
    where
        T: zigbee::Cluster + HeaderFactory + ToLeStream + Send;
}

impl<T> Transceiver for T
where
    T: Transmitter + DemuxProxy + Sync,
{
    async fn communicate<U>(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        frame: U,
    ) -> Result<Frame<Cluster>, Error>
    where
        U: zigbee::Cluster + HeaderFactory + ToLeStream + Send,
    {
        let seq = self.next_seq().await?;
        let response = self.subscribe(seq).await?;
        self.send(pan_id, endpoint, frame.frame(seq)).await?;
        response.recv().await
    }
}
