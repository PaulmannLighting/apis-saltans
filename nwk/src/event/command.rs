/// Commands received on the APS layer.
#[expect(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
    /// A ZDP aps was received.
    // TODO: Is theis ZDP or ZDO?
    Zdp(zdp::Frame<zdp::Command>),
    /// A ZCL command was received.
    Zcl(zcl::Frame<zcl::Cluster>),
}
