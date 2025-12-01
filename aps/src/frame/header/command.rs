use le_stream::{FromLeStream, ToLeStream};

use crate::{Control, FrameType};

/// APS Command frame header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Command {
    control: Control,
    counter: u8,
}

impl Command {
    /// Creates a new APS Command frame header without any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `control` is consistent with a Command frame.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(control: Control, counter: u8) -> Self {
        Self { control, counter }
    }

    /// Creates a new APS Command frame header.
    #[must_use]
    pub fn new(counter: u8) -> Self {
        let mut control = Control::empty();
        control.set_frame_type(FrameType::Command);
        control.insert(Control::ACK_FORMAT);
        Self { control, counter }
    }

    pub(crate) fn from_le_stream_with_control<T>(control: Control, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let counter = u8::from_le_stream(&mut bytes)?;
        Some(Self { control, counter })
    }
}
