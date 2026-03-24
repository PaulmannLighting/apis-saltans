//! Header definitions for a generic APS Data frame.

use le_stream::{FromLeStream, ToLeStream};

use crate::frame::data::unicast;
use crate::{Control, DeliveryMode, Destination, Extended, FrameType};

/// A data frame header.
#[derive(Clone, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct Header {
    control: Control,
    destination: Destination,
    cluster_id: u16,
    profile_id: u16,
    source_endpoint: u8,
    counter: u8,
    extended: Option<Extended>,
}

impl Header {
    /// Create a new `Header`.
    pub fn new(
        destination: Destination,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        counter: u8,
        extended: Option<Extended>,
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
        }
    }

    /// Return the control field.
    #[must_use]
    pub const fn control(&self) -> Control {
        self.control
    }

    /// Return the destination.
    #[must_use]
    pub const fn destination(&self) -> Destination {
        self.destination
    }

    /// Return the cluster ID.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Return the profile ID.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Return the source endpoint.
    #[must_use]
    pub const fn source_endpoint(&self) -> u8 {
        self.source_endpoint
    }

    /// Return the APS frame counter.
    #[must_use]
    pub const fn counter(&self) -> u8 {
        self.counter
    }

    /// Return the extended header.
    #[must_use]
    pub const fn extended(&self) -> Option<Extended> {
        self.extended
    }
}

impl From<unicast::Header> for Header {
    fn from(header: unicast::Header) -> Self {
        Self {
            control: header.control(),
            destination: Destination::Unicast(header.dst_endpoint()),
            cluster_id: header.cluster_id(),
            profile_id: header.profile_id(),
            source_endpoint: header.source_endpoint(),
            counter: header.counter(),
            extended: header.extended(),
        }
    }
}

impl FromLeStream for Header {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;

        let destination = match control.delivery_mode() {
            Some(delivery_mode) => match delivery_mode {
                DeliveryMode::Unicast => Destination::Unicast(u8::from_le_stream(&mut bytes)?),
                DeliveryMode::Broadcast => Destination::Broadcast(u8::from_le_stream(&mut bytes)?),
                DeliveryMode::Group => Destination::Group(u16::from_le_stream(&mut bytes)?),
            },
            None => return None, // TODO: Do we have a better option to handle the error case here?
        };

        let cluster_id = u16::from_le_stream(&mut bytes)?;
        let profile_id = u16::from_le_stream(&mut bytes)?;
        let source_endpoint = u8::from_le_stream(&mut bytes)?;
        let counter = u8::from_le_stream(&mut bytes)?;

        let extended = if control.contains(Control::EXTENDED_HEADER) {
            Some(Extended::from_le_stream(
                matches!(control.frame_type(), FrameType::Acknowledgment),
                &mut bytes,
            )?)
        } else {
            None
        };

        Some(Self {
            control,
            destination,
            cluster_id,
            profile_id,
            source_endpoint,
            counter,
            extended,
        })
    }
}
