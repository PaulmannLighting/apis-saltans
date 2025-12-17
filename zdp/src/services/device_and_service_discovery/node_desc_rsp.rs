use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;
use zigbee::node::Descriptor;
use zigbee::types::tlv::Tlv;

use crate::{Displayable, Service, Status};

/// Node Descriptor Response structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct NodeDescRsp {
    nwk_addr_of_interest: u16,
    status: u8,
    node_descriptor: Descriptor,
    tlvs: Vec<Tlv>,
}

impl NodeDescRsp {
    /// Creates a new `NodeDescRsp`.
    #[must_use]
    pub const fn new(
        nwk_addr_of_interest: u16,
        status: Status,
        node_descriptor: Descriptor,
        tlvs: Vec<Tlv>,
    ) -> Self {
        Self {
            nwk_addr_of_interest,
            status: status as u8,
            node_descriptor,
            tlvs,
        }
    }

    /// Returns the network address of interest.
    #[must_use]
    pub const fn nwk_addr_of_interest(&self) -> u16 {
        self.nwk_addr_of_interest
    }

    /// Returns the status.
    ///
    /// # Errors
    ///
    /// Returns an error if the status value is not valid.
    pub fn status(&self) -> Result<Status, u8> {
        self.status.try_into()
    }

    /// Returns the node descriptor.
    #[must_use]
    pub const fn node_descriptor(&self) -> &Descriptor {
        &self.node_descriptor
    }

    /// Returns the TLVs.
    #[must_use]
    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }
}

impl Cluster for NodeDescRsp {
    const ID: u16 = 0x8002;
}

impl Service for NodeDescRsp {
    const NAME: &'static str = "Node_Desc_rsp";
}

impl Display for NodeDescRsp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {{ nwk_addr_of_interest: {:#06X}, status: {}, node_descriptor: {:?}, tlvs: [",
            Self::NAME,
            self.nwk_addr_of_interest,
            self.status().display(),
            self.node_descriptor
        )?;

        let mut tlvs = self.tlvs.iter();

        if let Some(tlv) = tlvs.next() {
            write!(f, "{tlv:?}")?;

            for tlv in tlvs {
                write!(f, ", {tlv:?}")?;
            }
        }

        write!(f, "] }}")
    }
}
