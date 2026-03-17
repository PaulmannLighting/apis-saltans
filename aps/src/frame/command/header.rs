use le_stream::{FromLeStream, ToLeStream};

use crate::Control;

/// A header for APS Command frames.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Header {
    control: Control,
    counter: u8,
    id: u8,
}

impl Header {
    /// Create a new `Header`.
    #[must_use]
    pub fn new(control: Control, counter: u8, id: u8) -> Self {
        Self {
            control,
            counter,
            id,
        }
    }

    /// Return the control field.
    #[must_use]
    pub const fn control(&self) -> Control {
        self.control
    }

    /// Return the APS counter.
    #[must_use]
    pub const fn counter(&self) -> u8 {
        self.counter
    }

    /// Return the APS command ID.
    #[must_use]
    pub const fn id(&self) -> u8 {
        self.id
    }
}
