use le_stream::FromLeStream;
use zigbee::Cluster;
use zigbee::types::tlv::Tlv;

use crate::Service;

/// Node Descriptor Request structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct NodeDescReq {
    nwk_addr: u16,
    tlvs: Vec<Tlv>,
}

impl NodeDescReq {
    /// Creates a new `NodeDescReq`.
    #[must_use]
    pub const fn new(nwk_addr: u16, tlvs: Vec<Tlv>) -> Self {
        Self { nwk_addr, tlvs }
    }

    /// Returns the network address.
    #[must_use]
    pub const fn nwk_addr(&self) -> u16 {
        self.nwk_addr
    }

    /// Returns the TLVs.
    #[must_use]
    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }
}

impl Cluster for NodeDescReq {
    const ID: u16 = 0x0002;
}

impl Service for NodeDescReq {
    const NAME: &'static str = "Node_Desc_req";
}
