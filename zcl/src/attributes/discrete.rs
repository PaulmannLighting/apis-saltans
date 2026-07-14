use std::marker::PhantomData;

/// Reporting parameters for a discrete attribute.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Discrete<T> {
    pub(crate) minimum_reporting_interval: u16,
    pub(crate) maximum_reporting_interval: u16,
    phantom_data: PhantomData<T>,
}

impl<T> Discrete<T> {
    /// Creates reporting parameters for a discrete attribute.
    #[must_use]
    pub const fn new(minimum_reporting_interval: u16, maximum_reporting_interval: u16) -> Self {
        Self {
            minimum_reporting_interval,
            maximum_reporting_interval,
            phantom_data: PhantomData,
        }
    }
}
