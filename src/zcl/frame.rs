use crate::zcl::Command;

pub use header::{Control, Direction, Header, Type};

mod header;

/// A ZCL frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T>
where
    T: Command,
{
    /// Creates a new ZCL frame.
    #[must_use]
    pub const fn new(
        control: Control,
        manufacturer_code: Option<u16>,
        seq: u8,
        payload: T,
    ) -> Self {
        Self {
            header: Header::new(control, manufacturer_code, seq, T::ID),
            payload,
        }
    }

    /// Returns the header of the ZCL frame.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Returns the payload of the ZCL frame.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }
}
