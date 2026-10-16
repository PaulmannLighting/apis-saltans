use core::fmt::Formatter;

use zb_core::Eui64;
use zb_core::types::tlv::{Local, Tlv};

use crate::ClearAllBindingsRsp;
use crate::storage::TlvSlice;

crate::zdp_command! {
    /// Clear All Bindings Request
    ClearAllBindingsReq => Clear_All_Bindings_req;
    cluster_id: 0x002b;
    group: BindManagement;
    response: ClearAllBindingsRsp;
    fields {
        tlvs: TlvSlice,
    }
    getters {
        /// Returns the provided TLVs.
        #[must_use]
        pub fn tlvs(&self) -> &[Tlv] {
            &self.tlvs
        }

        /// Returns an iterator over all EUI64s in the Clear All Bindings Request.
        pub fn eui64s(&self) -> impl Iterator<Item = &'_ Eui64> {
            self.tlvs
                .iter()
                .filter_map(|tlv| {
                    if let Tlv::Local(
                        Local::ClearAllBindingsReqEui64(clear_all_bindings_req_eui64),
                    ) = tlv
                    {
                        Some(clear_all_bindings_req_eui64.eui64s())
                    } else {
                        None
                    }
                })
                .flatten()
        }
    }
    display {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f, "{} {{ tlvs: {:?} }}", Self::NAME, self.tlvs)
        }
    }
}
