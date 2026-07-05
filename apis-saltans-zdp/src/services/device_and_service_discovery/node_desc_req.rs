use apis_saltans_core::types::tlv::{FragmentationParameters, Global, Tlv};

use crate::NodeDescRsp;

crate::services::zdp_command! {
    /// Node Descriptor Request structure.
    NodeDescReq => Node_Desc_req;
    cluster_id: 0x0002;
    group: DeviceAndServiceDiscovery;
    response: NodeDescRsp;
    fields {
        nwk_addr: u16,
        tlvs: Vec<Tlv>,
    }
    constructor {
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
    }
    getters {
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
    display {
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
    from {
        impl From<FragmentationParameters> for NodeDescReq {
            fn from(fragmentation: FragmentationParameters) -> Self {
                Self {
                    nwk_addr: fragmentation.node_id(),
                    tlvs: vec![Tlv::Global(Global::FragmentationParameters(fragmentation))],
                }
            }
        }
    }
}
