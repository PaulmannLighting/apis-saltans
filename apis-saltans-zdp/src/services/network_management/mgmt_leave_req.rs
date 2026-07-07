use apis_saltans_core::IeeeAddress;

pub use self::leave_req_flags::LeaveReqFlags;

mod leave_req_flags;

crate::zdp_command! {
    /// Management Leave Request structure.
    MgmtLeaveReq => Mgmt_Leave_req;
    cluster_id: 0x0034;
    group: NetworkManagement;
    response: crate::MgmtLeaveRsp;
    fields {
        device_address: IeeeAddress,
        flags: LeaveReqFlags,
    }
    getters {
        /// Returns the device address.
        #[must_use]
        pub const fn device_address(&self) -> IeeeAddress {
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
