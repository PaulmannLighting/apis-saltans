use crate::IeeeAddrReq;

/// Available ZDP frames.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Frames {
    /// IEEE Address Request
    IeeeAddrReq(IeeeAddrReq),
}
