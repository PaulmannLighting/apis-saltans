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
    DataIndication {
        /// APS data-service indication containing the incoming ASDU.
        indication: DataIndication<Bytes, T, K>,

        /// Whether the application must respond to an incoming ZDO request.
        ///
        /// Backends must set this from the NCP's incoming-message metadata. The value is ignored
        /// for messages outside the Zigbee Device Profile.
        zdo_response_required: bool,
    },

    /// Completion of an accepted acknowledged APS transmission.
    DataConfirm {
        /// APS counter supplied with the corresponding transmission request.
        counter: u8,

        /// APS data-service confirmation.
        confirmation: DataConfirm<T>,
    },
}
