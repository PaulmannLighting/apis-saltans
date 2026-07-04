crate::services::zdp_command! {
    /// Request type for IEEE address request.
    IeeeAddrReq => IEEE_addr_req;
    cluster_id: 0x0001;
    fields {
        nwk_addr_of_interest: u16,
        request_type: u8,
        start_index: u8,
    }
    getters {
        /// Returns the network address of interest.
        #[must_use]
        pub const fn nwk_addr_of_interest(&self) -> u16 {
            self.nwk_addr_of_interest
        }

        /// Returns the request type.
        #[must_use]
        pub const fn request_type(&self) -> u8 {
            self.request_type
        }

        /// Returns the start index.
        #[must_use]
        pub const fn start_index(&self) -> u8 {
            self.start_index
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ nwk_addr_of_interest: {:#06X}, request_type: {:#04X}, start_index: {:#04X} }}",
                Self::NAME,
                self.nwk_addr_of_interest,
                self.request_type,
                self.start_index
            )
        }
    }
}
