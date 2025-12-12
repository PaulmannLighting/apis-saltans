use le_stream::{FromLeStream, ToLeStream};
use macaddr::MacAddr8;
use zigbee::Cluster;

pub use self::leave_req_flags::LeaveReqFlags;
use crate::Service;

mod leave_req_flags;

/// Management Leave Request structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtLeaveReq {
    device_address: MacAddr8,
    flags: LeaveReqFlags,
}

impl MgmtLeaveReq {
    /// Creates a new `MgmtLeaveReq`.
    #[must_use]
    pub const fn new(device_address: MacAddr8, flags: LeaveReqFlags) -> Self {
        Self {
            device_address,
            flags,
        }
    }

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

impl Cluster for MgmtLeaveReq {
    const ID: u16 = 0x0034;
}

impl Service for MgmtLeaveReq {
    const NAME: &'static str = "Mgmt_Leave_req";
}
