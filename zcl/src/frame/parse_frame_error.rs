/// Frame parsing error.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ParseFrameError {
    /// The ZCL frame header is invalid.
    InvalidHeader,
    /// Invalid cluster ID.
    InvalidClusterId(u16),
    /// Invalid command ID.
    InvalidCommandId(u8),
    /// The amount of bytes of the payload is insufficient.
    InsufficientPayload,
}
