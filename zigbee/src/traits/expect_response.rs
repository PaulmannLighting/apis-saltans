use core::fmt::Debug;

/// Trait for commands that are expected to respond with a specific response type.
pub trait ExpectResponse<T>: Into<T> {
    /// The response type.
    type Response: TryFrom<T, Error: Debug>;
}
