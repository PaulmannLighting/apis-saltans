use zcl::Cluster;
use zdp::Command;

/// Received frame payload.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Payload {
    /// ZCL payload.
    Zcl {
        /// An optional manufacturer code.
        manufacturer_code: Option<u16>,
        /// ZCL payload.
        payload: Cluster,
    },
    /// ZDP payload.
    Zdp(Command),
}
