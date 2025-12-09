use crate::IeeeAddrReq;

/// Available ZDP frames.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Frames {
    /// IEEE Address Request
    IeeeAddrReq(IeeeAddrReq),
}
