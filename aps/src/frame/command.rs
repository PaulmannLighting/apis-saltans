//! APS Command Frame.

use le_stream::{FromLeStream, ToLeStream};

use crate::Control;

/// APS Command Frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct Frame<T> {
    control: Control,
    counter: u8,
    id: u8,
    payload: T,
}

impl<T> Frame<T> {
    /// Create a new command aps.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `control` and `id` are consistent with a Command aps.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(control: Control, counter: u8, id: u8, payload: T) -> Self {
        Self {
            control,
            counter,
            id,
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

    /// Consumes the command aps and returns the payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }
}

impl<T> Frame<T>
where
    T: FromLeStream,
{
    /// Creates a new APS Command aps from a little-endian byte stream with the given control field.
    pub fn from_le_stream_with_control<U>(control: Control, mut bytes: U) -> Option<Self>
    where
        U: Iterator<Item = u8>,
    {
        let counter = u8::from_le_stream(&mut bytes)?;
        let id = u8::from_le_stream(&mut bytes)?;
        let payload = T::from_le_stream(&mut bytes)?;

        Some(Self {
            control,
            counter,
            id,
            payload,
        })
    }
}
