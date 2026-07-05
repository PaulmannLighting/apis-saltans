crate::zdp_command! {
    /// Management LQI Request structure.
    derive { Copy }
    MgmtLqiReq => Mgmt_Lqi_req;
    cluster_id: 0x0031;
    group: NetworkManagement;
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
}
