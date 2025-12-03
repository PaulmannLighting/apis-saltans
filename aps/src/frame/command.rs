use le_stream::ToLeStream;

use crate::frame::destination::Destination;
use crate::{Control, DeliveryMode, FrameType};

/// APS Command Frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct Command<T> {
    control: Control,
    counter: u8,
    id: u8,
    payload: T,
}

impl<T> Command<T> {
    /// Create a new command frame.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `control` and `id` are consistent with a Command frame.
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

    /// Consumes the command frame and returns the payload.
    #[must_use]
    pub fn into_payload(self) -> T {
        self.payload
    }
}

impl<T> Command<T>
where
    T: zcl::Command,
{
    /// Creates a new APS Command frame.
    #[must_use]
    pub const fn new(destination: Destination, counter: u8, payload: T) -> Self {
        let mut control = Control::empty();
        control.set_frame_type(FrameType::Command);

        match destination {
            Destination::Unicast { .. } => {
                control.set_delivery_mode(DeliveryMode::Unicast);
            }
            Destination::Broadcast(_) => {
                control.set_delivery_mode(DeliveryMode::Broadcast);
            }
            Destination::Group(_) => {
                control.set_delivery_mode(DeliveryMode::Group);
            }
        }

        Self {
            control,
            counter,
            id: <T as zcl::Command>::ID,
            payload,
        }
    }
}
