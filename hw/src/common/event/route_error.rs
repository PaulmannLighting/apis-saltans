use thiserror::Error;

/// Route errors.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Ord, PartialOrd, Hash)]
pub enum RouteError {
    /// A source address routing error.
    #[error("Source address {0:#06X} not found")]
    Source(u16),

    /// A many-to-one routing error.
    #[error("Many-to-one routing error for address: {0:#06X}")]
    ManyToOne(u16),
}
