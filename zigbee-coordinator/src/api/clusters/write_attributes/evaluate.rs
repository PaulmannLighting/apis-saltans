use zcl::global::write_attributes::Response;

/// Extension trait to evaluate a [`Response`].
pub trait Evaluate {
    /// Evaluate a [`Response`] and return an error if any occurred.
    fn evaluate(self) -> Vec<Result<u16, u16>>;
}

impl Evaluate for Response {
    fn evaluate(self) -> Vec<Result<u16, u16>> {
        self.into_iter().map(TryInto::try_into).collect()
    }
}
