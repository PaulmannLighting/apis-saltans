use le_stream::{FromLeStream, ToLeStream};

use crate::Control;

/// APS Command frame header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Command {
    control: Control,
    counter: u8,
}

impl Command {
    pub(crate) fn from_le_stream_with_control<T>(control: Control, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let counter = u8::from_le_stream(&mut bytes)?;
        Some(Self { control, counter })
    }
}
