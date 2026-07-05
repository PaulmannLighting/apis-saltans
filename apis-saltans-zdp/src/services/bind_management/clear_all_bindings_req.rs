use apis_saltans_core::types::tlv::{Local, Tlv};
use macaddr::MacAddr8;

crate::zdp_command! {
    /// Clear All Bindings Request
    ClearAllBindingsReq => Clear_All_Bindings_req;
    cluster_id: 0x002b;
    group: BindManagement;
    fields {
        tlvs: Box<[Tlv]>,
    }
    getters {
        /// Returns the provided TLVs.
        #[must_use]
        pub fn tlvs(&self) -> &[Tlv] {
            &self.tlvs
        }

        /// Returns an iterator over all EUI64s in the Clear All Bindings Request.
        pub fn eui64s(&self) -> impl Iterator<Item = &'_ MacAddr8> {
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {{ tlvs: {:?} }}", Self::NAME, self.tlvs)
        }
    }
}
