/// Command type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Type {
    /// A global command.
    Global = 0x00,
    /// A cluster-specific command.
    ClusterSpecific = 0x01,
}
