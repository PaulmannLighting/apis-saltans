use zb_aps::apsde::{DataIndication, DataRequest, NetworkAddress, Source};
use zb_core::{Direction, Endpoint, short_id};
use zb_zdp::{CLUSTER_ID_RESPONSE_MASK, Command};

/// Correlation key for pending transceiver responses.
///
/// The coordinator stores outstanding ZCL and ZDP requests under an `Index` and
/// removes the matching entry again when a response frame arrives. The key uses
/// the addressing and protocol fields that are expected to be mirrored by the
/// response: the remote node id, endpoint, cluster id, profile id, optional
/// manufacturer code, expected ZCL direction where applicable, and transaction sequence number.
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
    /// Expected ZCL response direction, or `None` for directionless ZDP exchanges.
    direction: Option<Direction>,
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
            direction: None,
            seq,
        }
    }

    /// Create a ZCL response-correlation key with its expected frame direction.
    #[must_use]
    pub const fn new_zcl(
        short_id: u16,
        endpoint: Endpoint,
        cluster_id: u16,
        profile_id: u16,
        manufacturer_code: Option<u16>,
        direction: Direction,
        seq: u8,
    ) -> Self {
        Self {
            short_id,
            endpoint,
            cluster_id,
            profile_id,
            manufacturer_code,
            direction: Some(direction),
            seq,
        }
    }

    /// Return the transaction sequence represented by this key.
    #[must_use]
    pub const fn sequence(self) -> u8 {
        self.seq
    }

    /// Create the response correlation key for a sent ZDP command.
    ///
    /// ZDP commands are exchanged on the data endpoint and do not carry a ZCL
    /// manufacturer code, so the key is built from the request's cluster and
    /// profile identifiers plus the transaction sequence number.
    #[must_use]
    pub fn from_zdp_command<T>(
        device: short_id::Device,
        seq: u8,
        request: &DataRequest<T>,
    ) -> Self {
        Self::new(
            device.into(),
            Endpoint::Data,
            request.cluster_id(),
            request.profile_id(),
            None,
            seq,
        )
    }

    /// Create the response correlation key for a received ZCL indication.
    ///
    /// Returns `None` when the indication source is not a 16-bit NWK address with an endpoint.
    #[must_use]
    pub const fn from_received_zcl_indication<T, K>(
        indication: &DataIndication<zb_zcl::Frame<zb_zcl::Cluster>, T, K>,
    ) -> Option<Self> {
        let Source::Network {
            address: source,
            endpoint,
        } = indication.metadata().source()
        else {
            return None;
        };
        let header = indication.asdu().header();

        Some(Self::new_zcl(
            source.as_u16(),
            endpoint.get(),
            indication.metadata().cluster_id(),
            indication.metadata().profile_id(),
            header.manufacturer_code(),
            header.control().direction(),
            header.seq(),
        ))
    }

    /// Create the response correlation key for a received ZDP response frame.
    ///
    /// ZDP response cluster ids carry [`CLUSTER_ID_RESPONSE_MASK`]. The mask is
    /// toggled away before indexing so the response matches the key that was
    /// stored for the original request command.
    #[must_use]
    pub fn from_received_zdp_frame(source: NetworkAddress, frame: &zb_zdp::Frame<Command>) -> Self {
        Self::new(
            source.as_u16(),
            Endpoint::Data,
            frame.data().cluster_id() ^ CLUSTER_ID_RESPONSE_MASK,
            frame.data().profile().into(),
            None,
            frame.seq(),
        )
    }
}
