/// Trait for commands that is expected to respond with a specfic response type.
pub trait ExpectResponse<T>: Into<T> {
    /// The response type.
    type Response: TryFrom<T>;
}
