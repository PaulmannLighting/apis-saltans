use std::error::Error;
use std::fmt::Display;

/// Route errors.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RouteError {
    /// A source address routing error.
    Source(u16),

    /// A many-to-one routing error.
    ManyToOne(u16),
}

impl Display for RouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Source(short_id) => write!(f, "Source address {short_id:#06X} not found"),
            Self::ManyToOne(short_id) => {
                write!(f, "Many-to-one routing error for address: {short_id:#06X}")
            }
        }
    }
}

impl Error for RouteError {}
