/// Trait for commands that respond with a specfic response type.
pub trait RespondsWith {
    /// The response type.
    type Response;
}
