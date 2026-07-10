use zb_core::IeeeAddress;
use zb_core::node::MacCapabilityFlags;

crate::zdp_command! {
    /// Device Announcement Service.
    DeviceAnnce => Device_annce;
    cluster_id: 0x0013;
    group: DeviceAndServiceDiscovery;
    fields {
        nwk_addr: u16,
        ieee_addr: IeeeAddress,
        capabilities: MacCapabilityFlags,
    }
    getters {
        /// Returns the network address.
        #[must_use]
        pub const fn nwk_addr(&self) -> u16 {
            self.nwk_addr
        }

        /// Returns the IEEE address.
        #[must_use]
        pub const fn ieee_addr(&self) -> IeeeAddress {
            self.ieee_addr
        }

        /// Returns the capabilities.
        #[must_use]
        pub const fn capabilities(&self) -> MacCapabilityFlags {
            self.capabilities
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ nwk_addr: {:#06X}, ieee_addr: {}, capabilities: {} }}",
                Self::NAME,
                self.nwk_addr,
                self.ieee_addr,
                self.capabilities
            )
        }
    }
}
