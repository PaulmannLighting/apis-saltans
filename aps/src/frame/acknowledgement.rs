//! APS Acknowledgment Frame.

use le_stream::ToLeStream;

pub use self::ack_fmt::AckFmt;
use crate::{Control, Extended, FrameType};

mod ack_fmt;

/// APS Acknowledgment Frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Frame {
    control: Control,
    fmt: Option<AckFmt>, // Present if "ack format" is NOT set in control.
    counter: u8,
    extended: Option<Extended>,
}

impl Frame {
    /// Creates a new APS Acknowledgment frame header without any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `control` is consistent with an Acknowledgment frame.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(
        control: Control,
        fmt: Option<AckFmt>,
        counter: u8,
        extended: Option<Extended>,
    ) -> Self {
        Self {
            control,
            fmt,
            counter,
            extended,
        }
    }

    /// Creates a new APS Acknowledgment frame header.
    #[must_use]
    pub fn new(counter: u8, fmt: Option<AckFmt>, extended: Option<Extended>) -> Self {
        let mut control = Control::empty();
        control.set_frame_type(FrameType::Acknowledgment);

        if fmt.is_none() {
            control.insert(Control::ACK_FORMAT);
        }

        if extended.is_some() {
            control.insert(Control::EXTENDED_HEADER);
        }

        Self {
            control,
            fmt,
            counter,
            extended,
        }
    }

    /// Returns the control field.
    #[must_use]
    pub const fn control(&self) -> Control {
        self.control
    }

    /// Returns the acknowledgment format field.
    #[must_use]
    pub const fn fmt(&self) -> Option<AckFmt> {
        self.fmt
    }

    /// Returns the counter field.
    #[must_use]
    pub const fn counter(&self) -> u8 {
        self.counter
    }

    /// Returns the extended header field.
    #[must_use]
    pub const fn extended(&self) -> Option<Extended> {
        self.extended
    }
}
