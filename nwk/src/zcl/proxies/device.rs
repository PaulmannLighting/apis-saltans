use le_stream::ToLeStream;
use zcl::{Cluster, HeaderFactory};
use zigbee::Endpoint;

use crate::zcl::tx_rx::{Transceiver, Transmitter};
use crate::{Error, Frame};

/// Device-level ZCL proxy.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Proxy<T> {
    inner: T,
    pan_id: u16,
}

impl<T> Proxy<T> {
    /// Create a new device-level proxy.
    #[must_use]
    pub const fn new(inner: T, pan_id: u16) -> Self {
        Self { inner, pan_id }
    }

    /// Send a frame.
    pub async fn send<F>(&self, endpoint: Endpoint, frame: F) -> Result<u8, Error>
    where
        T: Transmitter,
        F: Into<Frame> + Send,
    {
        self.inner.send(self.pan_id, endpoint, frame).await
    }

    /// Send a frame and receive a response.
    pub async fn communicate<F>(
        &self,
        endpoint: Endpoint,
        frame: F,
    ) -> Result<zcl::Frame<Cluster>, Error>
    where
        T: Transceiver,
        F: zigbee::Cluster + HeaderFactory + ToLeStream + Send,
    {
        self.inner.communicate(self.pan_id, endpoint, frame).await
    }
}
