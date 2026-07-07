use apis_saltans_aps::Data;
use apis_saltans_core::{Application, Endpoint};
use apis_saltans_nwk::Source;
use apis_saltans_zcl::Cluster;
use apis_saltans_zdp::{CLUSTER_ID_RESPONSE_MASK, Command};

use crate::transceiver::zcl::Payload;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Index {
    short_id: u16,
    endpoint: Endpoint,
    cluster_id: u16,
    profile_id: u16,
    manufacturer_code: Option<u16>,
    seq: u8,
}

impl Index {
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

    #[must_use]
    pub const fn from_received_zcl_frame(
        source: Source,
        frame: &Data<apis_saltans_zcl::Frame<Cluster>>,
    ) -> Self {
        Self::from_aps_and_zcl_headers(source.node_id(), frame.header(), frame.payload().header())
    }

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
