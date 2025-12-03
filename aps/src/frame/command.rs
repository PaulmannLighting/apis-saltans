use le_stream::{FromLeStream, ToLeStream};

use crate::Control;

/// APS Command Frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct Command<T> {
    control: Control,
    counter: u8,
    payload: T,
}

impl<T> Command<T> {
    /// Create a new command frame.
    #[must_use]
    pub const fn new(control: Control, counter: u8, payload: T) -> Self {
        Self {
            control,
            counter,
            payload,
        }
    }

    /// Returns the control field.
    #[must_use]
    pub const fn control(&self) -> Control {
        self.control
    }

    /// Returns the counter.
    #[must_use]
    pub const fn counter(&self) -> u8 {
        self.counter
    }

    /// Returns a reference to the payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Consumes the command frame and returns the payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }
}

impl<T> Command<T>
where
    T: FromLeStream,
{
    pub(crate) fn from_le_stream_with_control<U>(control: Control, mut bytes: U) -> Option<Self>
    where
        U: Iterator<Item = u8>,
    {
        let counter = u8::from_le_stream(&mut bytes)?;
        let payload = T::from_le_stream(&mut bytes)?;

        Some(Self {
            control,
            counter,
            payload,
        })
    }
}
