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
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Header {
    /// APS Data frame header.
    Data(Data),
    /// APS Command frame header.
    Command(Command),
    /// APS Acknowledgment frame header.
    Acknowledgment(Acknowledgment),
}

#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Data {
    control: Control,
    destination: Destination,
    counter: u8,
}

#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Command {
    control: Control,
    counter: u8,
}

#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Acknowledgment {
    control: Control,
    inner: Option<AckInner>, // Present if "ack format" is NOT set in control.
    counter: u8,
    extended: Option<Extended>,
}
