use bytes::Bytes;
use zb_aps::apsde::{DataConfirm, DataIndication};

/// APS data-service events emitted by the hardware layer.
///
/// `T` is the backend-defined timestamp type used by confirmations and indications. `K` is the
/// backend-defined device-key-pair handle carried by link-key-secured indications.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ApsdeEvent<T = (), K = ()> {
    /// An incoming application-service data unit.
    DataIndication(DataIndication<Bytes, T, K>),

    /// Completion of an accepted acknowledged APS transmission.
    DataConfirm {
        /// APS counter supplied with the corresponding transmission request.
        counter: u8,

        /// APS data-service confirmation.
        confirmation: DataConfirm<T>,
    },
}
