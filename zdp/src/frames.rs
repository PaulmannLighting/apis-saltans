use crate::{BindReq, Frame, IeeeAddrReq, MgmtPermitJoiningReq, NwkAddrReq};

/// Available ZDP frames.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Frames {
    /// Bind Request
    BindReq(Frame<BindReq>),
    /// IEEE Address Request
    IeeeAddrReq(Frame<IeeeAddrReq>),
    /// Management Permit Joining Request
    MgmtPermitJoiningReq(Frame<MgmtPermitJoiningReq>),
    /// Network Address Request
    NwkAddrReq(Frame<NwkAddrReq>),
}
