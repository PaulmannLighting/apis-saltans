use le_stream::{FromLeStream, ToLeStream};

use crate::{Control, DeliveryMode, Destination, FrameType};

/// APS Data frame header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Data {
    control: Control,
    destination: Option<Destination>,
    counter: u8,
}

impl Data {
    /// Creates a new APS Data frame header.
    #[must_use]
    pub const fn new(destination: Destination, counter: u8) -> Self {
        let mut control = Control::empty();
        control.set_frame_type(FrameType::Data);

        match &destination {
            Destination::Unicast(_) => {
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
            destination: Some(destination),
            counter,
        }
    }

    pub(crate) fn from_le_stream_with_control<T>(control: Control, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let destination = if let Some(delivery_mode) = control.delivery_mode() {
            Some(match delivery_mode {
                DeliveryMode::Unicast => {
                    u8::from_le_stream(&mut bytes).map(Destination::Unicast)?
                }
                DeliveryMode::Broadcast => {
                    u8::from_le_stream(&mut bytes).map(Destination::Broadcast)?
                }
                DeliveryMode::Group => u16::from_le_stream(&mut bytes).map(Destination::Group)?,
            })
        } else {
            None
        };

        let counter = u8::from_le_stream(&mut bytes)?;

        Some(Self {
            control,
            destination,
            counter,
        })
    }
}
