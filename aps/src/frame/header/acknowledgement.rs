use le_stream::{FromLeStream, ToLeStream};

use self::ack_inner::AckInner;
use crate::{Control, Extended, FrameType};

pub mod ack_inner;

/// APS Acknowledgment frame header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Acknowledgment {
    control: Control,
    inner: Option<AckInner>, // Present if "ack format" is NOT set in control.
    counter: u8,
    extended: Option<Extended>,
}

impl Acknowledgment {
    pub(crate) fn from_le_stream_with_control<T>(control: Control, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let inner = if control.contains(Control::ACK_FORMAT) {
            None
        } else {
            Some(AckInner::from_le_stream(&mut bytes)?)
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
            inner,
            counter,
            extended,
        })
    }
}
