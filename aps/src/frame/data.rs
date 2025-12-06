use le_stream::FromLeStream;

use crate::frame::destination::Destination;
use crate::{Control, DeliveryMode, Extended, FrameType};

/// APS Data frame.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Data<T> {
    control: Control,
    destination: Destination,
    cluster_id: u16,
    profile_id: u16,
    source_endpoint: u8,
    counter: u8,
    extended: Option<Extended>,
    payload: T,
}

impl<T> Data<T> {
    /// Creates a new APS Data frame header without any validation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided `control` is consistent with a Data frame.
    #[expect(unsafe_code, clippy::too_many_arguments)]
    #[must_use]
    pub const unsafe fn new_unchecked(
        control: Control,
        destination: Destination,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        counter: u8,
        extended: Option<Extended>,
        payload: T,
    ) -> Self {
        Self {
            control,
            destination,
            cluster_id,
            profile_id,
            source_endpoint,
            counter,
            extended,
            payload,
        }
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
            control,
            destination,
            cluster_id,
            profile_id,
            source_endpoint,
            counter,
            extended,
            payload,
        }
    }
}

impl<T> Data<T>
where
    T: FromLeStream,
{
    /// Creates an APS Data frame from a little-endian byte stream, given the control field.
    pub fn from_le_stream_with_control<U>(control: Control, mut bytes: U) -> Option<Self>
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
            control,
            destination,
            cluster_id,
            profile_id,
            source_endpoint,
            counter,
            extended,
            payload,
        })
    }
}
