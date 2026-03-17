//! APS Data frame.

use le_stream::FromLeStream;

pub use self::header::Header;
use crate::frame::destination::Destination;
use crate::{Control, DeliveryMode, Extended, FrameType};

mod header;

/// APS Data frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Frame<T> {
    header: Header,
    payload: T,
}

impl<T> Frame<T> {
    /// Creates a new APS Data frame header without any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `control` is consistent with a Data frame.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(header: Header, payload: T) -> Self {
        Self { header, payload }
    }

    /// Creates a new APS Data frame header.
    #[must_use]
    pub fn new(
        destination: Destination,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        counter: u8,
        extended: Option<Extended>,
        payload: T,
    ) -> Self {
        let mut control = Control::empty();
        control.set_frame_type(FrameType::Data);
        control.set_destination(destination);

        if extended.is_some() {
            control.insert(Control::EXTENDED_HEADER);
        }

        Self {
            header: Header::new(
                control,
                destination,
                cluster_id,
                profile_id,
                source_endpoint,
                counter,
                extended,
            ),
            payload,
        }
    }

    /// Return a reference to the header.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// Return a reference to the payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Return the header and payload, consuming the frame.
    #[must_use]
    pub fn into_party(self) -> (Header, T) {
        (self.header, self.payload)
    }
}

impl<T> Frame<T>
where
    T: FromLeStream,
{
    /// Creates an APS Data frame from a little-endian byte stream, given the control field.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the control field indicates a valid Data frame.
    #[expect(unsafe_code)]
    pub unsafe fn from_le_stream_with_control<U>(control: Control, mut bytes: U) -> Option<Self>
    where
        U: Iterator<Item = u8>,
    {
        let destination = match control.delivery_mode()? {
            DeliveryMode::Unicast => Destination::Unicast(u8::from_le_stream(&mut bytes)?),
            DeliveryMode::Broadcast => Destination::Broadcast(u8::from_le_stream(&mut bytes)?),
            DeliveryMode::Group => Destination::Group(u16::from_le_stream(&mut bytes)?),
        };

        let cluster_id = u16::from_le_stream(&mut bytes)?;
        let profile_id = u16::from_le_stream(&mut bytes)?;
        let source_endpoint = u8::from_le_stream(&mut bytes)?;
        let counter = u8::from_le_stream(&mut bytes)?;

        let extended = if control.contains(Control::EXTENDED_HEADER) {
            Some(Extended::from_le_stream(false, &mut bytes)?)
        } else {
            None
        };

        let payload = T::from_le_stream(&mut bytes)?;

        Some(Self {
            header: Header::new(
                control,
                destination,
                cluster_id,
                profile_id,
                source_endpoint,
                counter,
                extended,
            ),
            payload,
        })
    }
}
