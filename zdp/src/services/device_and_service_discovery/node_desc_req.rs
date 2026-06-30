use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;
use zigbee::types::tlv::{FragmentationParameters, Global, Tlv};

use crate::Service;

/// Node Descriptor Request structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct NodeDescReq {
    nwk_addr: u16,
    tlvs: Vec<Tlv>,
}

impl NodeDescReq {
    /// Creates a new `NodeDescReq`.
    #[must_use]
    pub fn new(nwk_addr: u16, tlvs: Vec<Tlv>) -> Option<Self> {
        if !tlvs
            .iter()
            .any(|tlv| matches!(tlv, Tlv::Global(Global::FragmentationParameters(_))))
        {
            return None;
        }

        Some(Self { nwk_addr, tlvs })
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

impl Display for NodeDescReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ nwk_addr: {:#06X}, tlvs: {:?} }}",
            Self::NAME,
            self.nwk_addr,
            self.tlvs
        )
    }
}

impl From<FragmentationParameters> for NodeDescReq {
    fn from(fragmentation: FragmentationParameters) -> Self {
        Self {
            nwk_addr: fragmentation.node_id(),
            tlvs: vec![Tlv::Global(Global::FragmentationParameters(fragmentation))],
        }
    }
}
