crate::services::zdp_command! {
    /// Power Descriptor Request structure.
    derive { Copy }
    PowerDescReq => Power_Desc_req;
    cluster_id: 0x0003;
    group: DeviceAndServiceDiscovery;
    fields {
        nwk_addr_of_interest: u16,
    }
    getters {
        /// Returns the network address of interest.
        #[must_use]
        pub const fn nwk_addr_of_interest(self) -> u16 {
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
    from {
        impl From<PowerDescReq> for u16 {
            fn from(req: PowerDescReq) -> Self {
                req.nwk_addr_of_interest
            }
        }

        impl From<u16> for PowerDescReq {
            fn from(nwk_addr_of_interest: u16) -> Self {
                Self::new(nwk_addr_of_interest)
            }
        }
    }
}
