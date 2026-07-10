use std::fmt::{Display, UpperHex};

use crate::Status;

/// Extension trait to display a `Result<Status, u8>` in a user-friendly format.
pub trait Displayable {
    /// Returns something that implements `Display`.
    fn display(self) -> impl Display;
}

impl Displayable for Result<Status, u8> {
    fn display(self) -> impl Display {
        DisplayResult(self)
    }
}

#[derive(Debug)]
struct DisplayResult<T, E>(Result<T, E>);

impl<T, E> Display for DisplayResult<T, E>
where
    T: Display,
    E: UpperHex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Ok(status) => write!(f, "{status}"),
            Err(code) => write!(f, "RESERVED ({code:#04X})"),
        }
    }
}
