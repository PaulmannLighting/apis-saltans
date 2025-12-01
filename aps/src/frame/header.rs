use le_stream::{FromLeStream, ToLeStream};

use self::ack_inner::AckInner;
pub use self::control::{Control, DeliveryMode, FrameType};
pub use self::destination::Destination;
pub use self::extended::Extended;

mod ack_inner;
mod address;
mod control;
mod destination;
mod extended;

/// APS Frame Header.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Header {
    /// APS Data frame header.
    Data(Data),
    /// APS Command frame header.
    Command(Command),
    /// APS Acknowledgment frame header.
    Acknowledgment(Acknowledgment),
}

impl FromLeStream for Header {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;

        Some(match control.frame_type() {
            FrameType::Data => Data::from_le_stream_with_control(control, bytes).map(Self::Data)?,
            FrameType::Command => {
                Command::from_le_stream_with_control(control, &mut bytes).map(Self::Command)?
            }
            FrameType::Acknowledgment => {
                Acknowledgment::from_le_stream_with_control(control, bytes)
                    .map(Self::Acknowledgment)?
            }
            FrameType::InterPanAps => todo!("Implement Inter-PAN APS frame parsing."),
        })
    }
}

impl ToLeStream for Header {
    type Iter = Box<dyn Iterator<Item = u8>>;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Data(data) => Box::new(data.to_le_stream()),
            Self::Command(cmd) => Box::new(cmd.to_le_stream()),
            Self::Acknowledgment(ack) => Box::new(ack.to_le_stream()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Data {
    control: Control,
    destination: Option<Destination>,
    counter: u8,
}

impl Data {
    fn from_le_stream_with_control<T>(control: Control, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let destination = if let Some(delivery_mode) = control.delivery_mode() {
            Some(match delivery_mode {
                DeliveryMode::Unicast | DeliveryMode::Broadcast => {
                    u8::from_le_stream(&mut bytes).map(Destination::Endpoint)?
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

impl FromLeStream for Data {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;
        Self::from_le_stream_with_control(control, bytes)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Command {
    control: Control,
    counter: u8,
}

impl Command {
    fn from_le_stream_with_control<T>(control: Control, mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let counter = u8::from_le_stream(&mut bytes)?;
        Some(Self { control, counter })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Acknowledgment {
    control: Control,
    inner: Option<AckInner>, // Present if "ack format" is NOT set in control.
    counter: u8,
    extended: Option<Extended>,
}

impl Acknowledgment {
    fn from_le_stream_with_control<T>(control: Control, mut bytes: T) -> Option<Self>
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
            Some(Extended::from_le_stream(&mut bytes)?)
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

impl FromLeStream for Acknowledgment {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;
        Self::from_le_stream_with_control(control, bytes)
    }
}
