use le_stream::{FromLeStream, ToLeStream};

pub use self::acknowledgement::Acknowledgment;
pub use self::command::Command;
pub use self::control::{Control, DeliveryMode, FrameType};
pub use self::data::Data;
pub use self::destination::Destination;
pub use self::extended::Extended;

mod acknowledgement;
mod address;
mod command;
mod control;
mod data;
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

impl From<Data> for Header {
    fn from(data: Data) -> Self {
        Self::Data(data)
    }
}

impl From<Command> for Header {
    fn from(cmd: Command) -> Self {
        Self::Command(cmd)
    }
}

impl From<Acknowledgment> for Header {
    fn from(ack: Acknowledgment) -> Self {
        Self::Acknowledgment(ack)
    }
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
