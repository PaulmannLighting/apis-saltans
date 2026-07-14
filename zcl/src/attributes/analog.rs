/// Reporting parameters for an analog attribute.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Analog<T> {
    pub(crate) minimum_reporting_interval: u16,
    pub(crate) maximum_reporting_interval: u16,
    pub(crate) reportable_change: T,
}

impl<T> Analog<T> {
    /// Creates reporting parameters for an analog attribute.
    #[must_use]
    pub const fn new(
        minimum_reporting_interval: u16,
        maximum_reporting_interval: u16,
        reportable_change: T,
    ) -> Self {
        Self {
            minimum_reporting_interval,
            maximum_reporting_interval,
            reportable_change,
        }
    }
}
