/// Commands received on the APS layer.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
    /// A ZDP frame was received.
    // TODO: Is theis ZDP or ZDO?
    Aps(zdp::Frames),
    /// A ZCL command was received.
    Zcl(zcl::Frames),
}
