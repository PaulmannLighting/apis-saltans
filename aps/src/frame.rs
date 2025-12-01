pub use self::header::{
    Acknowledgment, Command, Control, Data, DeliveryMode, Destination, Extended, FrameType, Header,
};

mod header;

#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}
