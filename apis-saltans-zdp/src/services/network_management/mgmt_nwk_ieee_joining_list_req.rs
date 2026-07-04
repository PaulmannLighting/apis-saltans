crate::services::zdp_command! {
    /// Management Network IEEE Joining List Request.
    derive { Copy }
    MgmtNwkIeeeJoiningListReq => Mgmt_NWK_IEEE_Joining_List_req;
    cluster_id: 0x003A;
    fields {
        start_index: u8,
    }
    getters {
        /// Returns the start index.
        #[must_use]
        pub const fn start_index(self) -> u8 {
            self.start_index
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ start_index: {:#04X} }}",
                Self::NAME,
                self.start_index
            )
        }
    }
    from {
        impl From<u8> for MgmtNwkIeeeJoiningListReq {
            fn from(value: u8) -> Self {
                Self::new(value)
            }
        }

        impl From<MgmtNwkIeeeJoiningListReq> for u8 {
            fn from(value: MgmtNwkIeeeJoiningListReq) -> Self {
                value.start_index
            }
        }
    }
}
