use thiserror::Error;

/// Frame parsing error.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Hash)]
pub enum ParseFrameError {
    /// The ZCL frame header is invalid.
    #[error("Missing ZCL frame header")]
    MissingHeader,

    /// Invalid type field.
    #[error("Invalid type field: {0}")]
    InvalidType(u8),

    /// Invalid cluster ID.
    #[error("Invalid cluster ID: {0}")]
    InvalidClusterId(u16),

    /// Invalid command ID.
    #[error("Invalid command ID: {0}")]
    InvalidCommandId(u8),

    /// The number of bytes of the payload is not enough.
    #[error("Insufficient payload bytes")]
    InsufficientPayload,
}
