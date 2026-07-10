use crate::ActiveEpRsp;

crate::zdp_command! {
    /// Active Endpoint Request
    derive { Copy }
    ActiveEpReq => Active_EP_req;
    cluster_id: 0x0005;
    group: DeviceAndServiceDiscovery;
    response: ActiveEpRsp;
    fields {
        nwk_addr_of_interest: u16,
    }
    getters {
        /// Returns the network address of interest.
        #[must_use]
        pub const fn nwk_addr_of_interest(&self) -> u16 {
            self.nwk_addr_of_interest
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ nwk_addr_of_interest: {:#06X} }}",
                Self::NAME,
                self.nwk_addr_of_interest
            )
        }
    }
}
