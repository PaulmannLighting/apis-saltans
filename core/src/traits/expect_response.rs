use core::fmt::Debug;

/// Associates a request with its complete response type.
pub trait ExpectResponse<T> {
    /// The response type, including any alternative response forms accepted by the request.
    type Response: TryFrom<T, Error: Debug>;
}
