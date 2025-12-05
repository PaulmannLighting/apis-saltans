/// A ZDP client service.
pub trait Service {
    /// The name of the service.
    const NAME: &'static str;
}
