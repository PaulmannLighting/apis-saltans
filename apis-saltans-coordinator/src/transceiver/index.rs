use apis_saltans_aps::Data;
use apis_saltans_core::{Application, Endpoint};
use apis_saltans_nwk::Source;
use apis_saltans_zcl::Cluster;
use apis_saltans_zdp::{CLUSTER_ID_RESPONSE_MASK, Command};

use crate::transceiver::zcl::Payload;

/// Correlation key for pending transceiver responses.
///
/// The coordinator stores outstanding ZCL and ZDP requests under an `Index` and
/// removes the matching entry again when a response frame arrives. The key uses
/// the addressing and protocol fields that are expected to be mirrored by the
/// response: the remote node id, endpoint, cluster id, profile id, optional
/// manufacturer code, and transaction sequence number.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Index {
    /// The network short address of the remote node.
    short_id: u16,
    /// The endpoint used for the request/response exchange.
    endpoint: Endpoint,
    /// The request cluster id used for response matching.
    cluster_id: u16,
    /// The application profile id used for the exchange.
    profile_id: u16,
    /// The optional ZCL manufacturer code used by manufacturer-specific frames.
    manufacturer_code: Option<u16>,
    /// The transaction sequence number of the request/response exchange.
    seq: u8,
}

impl Index {
    /// Create a response correlation key from normalized response-matching
    /// fields.
    #[must_use]
    pub const fn new(
        short_id: u16,
        endpoint: Endpoint,
        cluster_id: u16,
        profile_id: u16,
        manufacturer_code: Option<u16>,
        seq: u8,
    ) -> Self {
        Self {
            short_id,
            endpoint,
            cluster_id,
            profile_id,
            manufacturer_code,
            seq,
        }
    }

    /// Create the response correlation key for a sent ZCL payload.
    ///
    /// The generated key uses the outbound APS addressing metadata together
    /// with the ZCL metadata and transaction sequence number. Incoming ZCL
    /// responses can then be matched by reconstructing the same key from their
    /// APS and ZCL headers.
    #[must_use]
    pub fn from_sent_payload<T>(
        short_id: u16,
        endpoint: Application,
        seq: u8,
        payload: &Payload<T>,
    ) -> Self {
        Self::new(
            short_id,
            endpoint.into(),
            payload.metadata().cluster_id(),
            payload.metadata().profile().into(),
            payload.manufacturer_code(),
            seq,
        )
    }

    /// Create the response correlation key for a sent ZDP command.
    ///
    /// ZDP commands are exchanged on the data endpoint and do not carry a ZCL
    /// manufacturer code, so the key is built from the command id, profile, and
    /// transaction sequence number.
    #[must_use]
    pub fn from_sent_command(short_id: u16, seq: u8, command: &Command) -> Self {
        Self::new(
            short_id,
            Endpoint::Data,
            command.cluster_id(),
            command.profile().into(),
            None,
            seq,
        )
    }

    /// Create the response correlation key for a received ZCL frame.
    ///
    /// The incoming frame contributes the APS and ZCL header fields, while the
    /// [`Source`] contributes the remote node id that sent the response.
    #[must_use]
    pub const fn from_received_zcl_frame(
        source: Source,
        frame: &Data<apis_saltans_zcl::Frame<Cluster>>,
    ) -> Self {
        Self::from_aps_and_zcl_headers(source.node_id(), frame.header(), frame.payload().header())
    }

    /// Create the response correlation key for a received ZDP response frame.
    ///
    /// ZDP response cluster ids carry [`CLUSTER_ID_RESPONSE_MASK`]. The mask is
    /// toggled away before indexing so the response matches the key that was
    /// stored for the original request command.
    #[must_use]
    pub fn from_received_zdp_frame(
        source: Source,
        frame: &apis_saltans_zdp::Frame<Command>,
    ) -> Self {
        Self::new(
            source.node_id(),
            Endpoint::Data,
            frame.data().cluster_id() ^ CLUSTER_ID_RESPONSE_MASK,
            frame.data().profile().into(),
            None,
            frame.seq(),
        )
    }

    /// Build the ZCL response correlation key from APS and ZCL headers.
    #[must_use]
    const fn from_aps_and_zcl_headers(
        short_id: u16,
        aps_header: &apis_saltans_aps::data::Header,
        zcl_header: apis_saltans_zcl::Header,
    ) -> Self {
        Self::new(
            short_id,
            aps_header.source_endpoint(),
            aps_header.cluster_id(),
            aps_header.profile_id(),
            zcl_header.manufacturer_code(),
            zcl_header.seq(),
        )
    }
}
