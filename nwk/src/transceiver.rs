use crate::{DemuxProxy, Proxy};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Transceiver<T, R> {
    tx_proxy: T,
    demux_proxy: R,
}

impl<T, R> Transceiver<T, R> {
    /// Create a new transceiver.
    #[must_use]
    pub const fn new(tx_proxy: T, demux_proxy: R) -> Self {
        Self {
            tx_proxy,
            demux_proxy,
        }
    }
}

impl<T, R> Transceiver<T, R>
where
    T: Proxy,
    R: DemuxProxy,
{
    fn zcl_transceive(&self) {
        todo!(
            "
                1) Obtain ZCL sequence number `seq`.
                2) Register handler with `seq`.
                3) Send message with `seq`.
                4) Receive response.
            "
        )
    }
}
