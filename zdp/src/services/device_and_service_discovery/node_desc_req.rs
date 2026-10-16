use core::fmt::Formatter;

use zb_core::types::tlv::{FragmentationParameters, Global, Tlv};

use crate::NodeDescRsp;
use crate::storage::Tlvs;

crate::zdp_command! {
    /// Node Descriptor Request structure.
    NodeDescReq => Node_Desc_req;
    cluster_id: 0x0002;
    group: DeviceAndServiceDiscovery;
    response: NodeDescRsp;
    fields {
        nwk_addr: u16,
        tlvs: Tlvs,
    }
    constructor {
        /// Creates a new `NodeDescReq`.
        #[must_use]
        pub fn new(nwk_addr: u16, tlvs: Tlvs) -> Option<Self> {
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
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
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
                    tlvs: crate::storage::single_tlv(Tlv::Global(Global::FragmentationParameters(fragmentation))),
                }
            }
        }
    }
}
