use bytes::Bytes;
use zb_aps::Data;
use zb_nwk::Envelope;

/// APS events emitted by the hardware layer.
#[derive(Clone, Debug)]
pub enum ApsEvent {
    /// Raw APS data frame received from a NWK source.
    MessageReceived(Envelope<Data<Bytes>>),

    /// Successful acknowledgement of an APS transmission.
    ///
    /// Contains the APS frame counter assigned to the acknowledged frame.
    Ack(u8),

    /// Failed acknowledgement of an APS transmission.
    Nak {
        /// APS frame counter assigned to the rejected frame.
        sequence: u8,

        /// Hardware error reported for the transmission.
        error: crate::Error,
    },
}
