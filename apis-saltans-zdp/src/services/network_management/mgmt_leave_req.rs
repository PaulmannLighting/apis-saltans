use macaddr::MacAddr8;

pub use self::leave_req_flags::LeaveReqFlags;

mod leave_req_flags;

crate::services::zdp_command! {
    /// Management Leave Request structure.
    MgmtLeaveReq => Mgmt_Leave_req;
    cluster_id: 0x0034;
    group: NetworkManagement;
    fields {
        device_address: MacAddr8,
        flags: LeaveReqFlags,
    }
    getters {
        /// Returns the device address.
        #[must_use]
        pub const fn device_address(&self) -> MacAddr8 {
            self.device_address
        }

        /// Returns the leave request flags.
        #[must_use]
        pub const fn flags(&self) -> LeaveReqFlags {
            self.flags
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {{ device_address: {}, flags: {} }}",
                Self::NAME,
                self.device_address,
                self.flags
            )
        }
    }
}
