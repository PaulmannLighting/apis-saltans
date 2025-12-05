use aps::{Acknowledgment, Command, Control, Data, FrameType};
use le_stream::FromLeStream;
use log::error;

/// An APS frame that has been received.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceivedApsFrame {
    /// Acknowledgment frame.
    Ack(Acknowledgment),
    /// Command frame.
    Command(Command<Box<[u8]>>),
    /// Data frame.
    Data(Data<Box<[u8]>>),
}

impl FromLeStream for ReceivedApsFrame {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;

        match control.frame_type() {
            FrameType::Acknowledgment => {
                Acknowledgment::from_le_stream_with_control(control, bytes).map(Self::Ack)
            }
            FrameType::Command => {
                Command::from_le_stream_with_control(control, bytes).map(Self::Command)
            }
            FrameType::Data => Data::from_le_stream_with_control(control, bytes).map(Self::Data),
            FrameType::InterPanAps => {
                error!("Received Inter-PAN APS frame, which is not supported.");
                None
            }
        }
    }
}
