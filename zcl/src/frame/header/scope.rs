use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Command scope.
#[derive(Clone, Copy, Debug, Eq, Hash, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum Scope {
    /// A global command.
    Global = 0x00,
    /// A cluster-specific command.
    ClusterSpecific = 0x01,
}
