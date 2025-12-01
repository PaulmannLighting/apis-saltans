use le_stream::{FromLeStream, ToLeStream};

use self::ack_fmt::AckFmt;
use crate::{Control, Extended, FrameType};

pub mod ack_fmt;

/// APS Acknowledgment frame header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Acknowledgment {
    control: Control,
    fmt: Option<AckFmt>, // Present if "ack format" is NOT set in control.
    counter: u8,
    extended: Option<Extended>,
}

impl Acknowledgment {
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

    pub(crate) fn from_le_stream_with_control<T>(control: Control, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let inner = if control.contains(Control::ACK_FORMAT) {
            None
        } else {
            Some(AckFmt::from_le_stream(&mut bytes)?)
        };

        let counter = u8::from_le_stream(&mut bytes)?;

        let extended = if control.contains(Control::EXTENDED_HEADER) {
            Some(Extended::from_le_stream(
                &mut bytes,
                control.frame_type() == FrameType::Acknowledgment,
            )?)
        } else {
            None
        };

        Some(Self {
            control,
            fmt: inner,
            counter,
            extended,
        })
    }
}
