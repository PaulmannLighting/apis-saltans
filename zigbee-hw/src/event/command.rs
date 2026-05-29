/// Commands received on the APS layer.
#[expect(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
    /// A ZDP frame was received.
    Zdp(zdp::Frame<zdp::Command>),
    /// A ZCL command was received.
    Zcl(zcl::Frame<zcl::Cluster>),
}

impl Command {
    /// Return the sequence number.
    #[must_use]
    pub const fn seq(&self) -> u8 {
        match self {
            Self::Zdp(zdp) => zdp.seq(),
            Self::Zcl(zcl) => zcl.header().seq(),
        }
    }
}
