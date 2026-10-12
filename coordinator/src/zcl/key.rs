use bytes::Bytes;
use zb_aps::apsde::{
    DataIndication, DataRequest, IndividualEndpoint, ReceivedDestination, RequestDestination,
    Source,
};
use zb_core::{Direction, Endpoint};
use zb_zcl::{Cluster, Frame, UnsequencedFrame};

use crate::Error;

/// Fields mirrored by a ZCL response, with endpoints relative to the coordinator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ZclKey {
    /// Network short address of the remote node.
    pub short_id: u16,
    /// Endpoint on the remote node.
    pub remote_endpoint: Endpoint,
    /// Endpoint on the coordinator.
    pub local_endpoint: Endpoint,
    /// Cluster used by the exchange.
    pub cluster_id: u16,
    /// Application profile used by the exchange.
    pub profile_id: u16,
    /// Manufacturer code for manufacturer-specific frames.
    pub manufacturer_code: Option<u16>,
    /// Expected direction of the response.
    pub direction: Direction,
    /// Transaction sequence number.
    pub sequence: u8,
}

impl ZclKey {
    /// Derive the expected response identity for an individual network destination.
    ///
    /// # Errors
    ///
    /// Returns an error unless the destination has a network address and individual endpoint.
    pub fn from_request(
        request: &DataRequest<UnsequencedFrame<Bytes>>,
        sequence_number: u8,
    ) -> Result<Self, Error> {
        let RequestDestination::Network { address, endpoint } = request.destination() else {
            return Err(Error::InvalidZclCommunicationDestination(
                request.destination(),
            ));
        };
        if IndividualEndpoint::new(endpoint).is_none() {
            return Err(Error::InvalidZclCommunicationDestination(
                request.destination(),
            ));
        }

        Ok(Self {
            short_id: address.as_u16(),
            remote_endpoint: endpoint,
            local_endpoint: request.source_endpoint().get(),
            cluster_id: request.cluster_id(),
            profile_id: request.profile_id(),
            manufacturer_code: request.asdu().header().manufacturer_code(),
            direction: !request.asdu().header().control().direction(),
            sequence: sequence_number,
        })
    }

    /// Create the response correlation key for a received ZCL indication.
    ///
    /// Returns `None` unless the source is a 16-bit NWK address with an endpoint and the
    /// destination is an individual address with a local endpoint.
    #[must_use]
    pub const fn from_indication<T, K>(
        indication: &DataIndication<Frame<Cluster>, T, K>,
    ) -> Option<Self> {
        let Source::Network {
            address: source,
            endpoint,
        } = indication.metadata().source()
        else {
            return None;
        };
        let local_endpoint = match indication.metadata().destination() {
            ReceivedDestination::Network { endpoint, .. }
            | ReceivedDestination::Extended { endpoint, .. } => endpoint.get(),
            _ => return None,
        };
        let header = indication.asdu().header();

        Some(Self {
            short_id: source.as_u16(),
            remote_endpoint: endpoint.get(),
            local_endpoint,
            cluster_id: indication.metadata().cluster_id(),
            profile_id: indication.metadata().profile_id(),
            manufacturer_code: header.manufacturer_code(),
            direction: header.control().direction(),
            sequence: header.seq(),
        })
    }
}
