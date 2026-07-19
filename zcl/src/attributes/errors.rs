use core::fmt::{self, Debug, Display};
use std::boxed::Box;

use thiserror::Error;
use zb_core::types::Type;

use crate::Status;

/// An error that occurs when parsing an attribute fails.
#[derive(Clone, Debug, Eq, Error, PartialEq, Hash)]
#[cfg_attr(target_pointer_width = "64", expect(variant_size_differences))]
pub enum ParseAttributeError<T> {
    /// The attribute is unsupported.
    #[error("unsupported attribute {id:#04X}: {}", display_status(.status))]
    Unsupported {
        /// The attribute ID.
        id: u16,
        /// The error status.
        status: Result<Status, u8>,
    },

    /// The attribute ID is invalid.
    #[error("Invalid attribute ID: {0}")]
    InvalidId(u16),

    /// The attribute type is invalid for this ID.
    #[error("{0}")]
    InvalidType(#[from] Box<InvalidType<T>>),
}

impl<T> From<InvalidType<T>> for ParseAttributeError<T> {
    fn from(error: InvalidType<T>) -> Self {
        Self::InvalidType(error.into())
    }
}

/// The data type is invalid for the given attribute ID.
#[derive(Clone, Debug, Eq, Error, PartialEq, Hash)]
#[error("Invalid type {typ:?} for attribute with id {id}")]
pub struct InvalidType<T> {
    id: T,
    typ: Type,
}

impl<T> InvalidType<T> {
    /// Create a new invalid type error.
    #[must_use]
    pub(crate) const fn new(id: T, typ: Type) -> Self {
        Self { id, typ }
    }
}

fn display_status(status: &Result<Status, u8>) -> impl Display + '_ {
    fmt::from_fn(|f| match status {
        Ok(status) => Debug::fmt(status, f),
        Err(status) => write!(f, "{status:#06X}"),
    })
}
