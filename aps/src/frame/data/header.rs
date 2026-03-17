//! Header definitions for a generic APS Data frame.

use crate::frame::data::unicast;
use crate::{Control, Destination, Extended, FrameType};

/// A data frame header.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    /// Create a new `Header`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    ///
    /// 1) the delivery mode corresponds to the destination and
    /// 2) that the extended flag corresponds to the presence of the extended header.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(
        control: Control,
        destination: Destination,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        counter: u8,
        extended: Option<Extended>,
    ) -> Self {
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
