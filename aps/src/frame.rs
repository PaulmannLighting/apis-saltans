use le_stream::FromLeStream;

pub use self::acknowledgement::AckFmt;
pub use self::control::{Control, DeliveryMode, FrameType};
pub use self::destination::Destination;
pub use self::extended::{Control as ExtendedControl, Extended, Fragmentation};

pub mod acknowledgement;
pub mod command;
mod control;
pub mod data;
mod destination;
mod extended;

/// A generic APS frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Frame<T> {
    /// The frame is an acknowledgement frame.
    Acknowledgement(acknowledgement::Frame),
    /// The frame is a command frame.
    Command(command::Frame<T>),
    /// The frame is a data frame.
    Data(data::Frame<T>),
}

impl<T> FromLeStream for Frame<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;

        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        // SAFETY: We ensure that the control field reflects the correct frame type
        // by means of the following match statement.
        match control.frame_type() {
            FrameType::Acknowledgment => {
                unsafe { acknowledgement::Frame::from_le_stream_with_control(control, bytes) }
                    .map(Self::Acknowledgement)
            }
            FrameType::Data => {
                unsafe { data::Frame::from_le_stream_with_control(control, bytes) }.map(Self::Data)
            }
            FrameType::Command => {
                unsafe { command::Frame::from_le_stream_with_control(control, bytes) }
                    .map(Self::Command)
            }
            FrameType::InterPanAps => unimplemented!("Inter-PAN APS frames are not yet supported."),
        }
    }
}
