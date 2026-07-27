use bytes::Bytes;
use zb_aps::Data;
use zb_nwk::Envelope;

use crate::TransmissionError;

/// APS events emitted by the hardware layer.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ApsEvent {
    /// Raw APS data frame received from a NWK source.
    MessageReceived(Envelope<Data<Bytes>>),

    /// An acknowledged APS transmission completed successfully.
    Ack(u8),

    /// An accepted APS transmission failed.
    Nak {
        /// APS counter of the failed transmission.
        sequence: u8,

        /// Hardware transmission failure.
        error: TransmissionError,
    },
}
