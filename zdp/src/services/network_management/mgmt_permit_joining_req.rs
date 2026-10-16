use core::fmt::Formatter;

use zb_core::types::tlv::Tlv;

use crate::MgmtPermitJoiningRsp;
use crate::storage::Tlvs;

crate::zdp_command! {
    /// Service for management permit joining request.
    MgmtPermitJoiningReq => Mgmt_Permit_Joining_req;
    cluster_id: 0x0036;
    group: NetworkManagement;
    response: MgmtPermitJoiningRsp;
    fields {
        duration: u8,
        tc_significance: bool,
        tlv_data: Tlvs,
    }
    getters {
        /// Returns the duration.
        #[must_use]
        pub const fn duration(&self) -> u8 {
            self.duration
        }

        /// Returns the TC significance.
        #[must_use]
        pub const fn tc_significance(&self) -> bool {
            self.tc_significance
        }

        /// Returns the TLV data.
        #[must_use]
        pub fn tlv_data(&self) -> &[Tlv] {
            &self.tlv_data
        }
    }
    display {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(
                f,
                "{} {{ duration: {:#04X}, tc_significance: {}, tlv_data: {:?} }}",
                Self::NAME,
                self.duration,
                self.tc_significance,
                self.tlv_data
            )
        }
    }
}
