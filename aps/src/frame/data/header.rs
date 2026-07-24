//! Header definitions for a generic APS Data frame.

use le_stream::{FromLeStream, ToLeStream};
use zb_core::{Cluster, Endpoint, Profile};

use crate::frame::destination::Destination;
use crate::{Control, Extended, Fragmentation, FrameType};

/// A data frame header.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ToLeStream)]
pub struct Header {
    control: Control,
    destination: Destination,
    cluster_id: u16,
    profile_id: u16,
    source_endpoint: Endpoint,
    counter: u8,
    extended: Option<Extended>,
}

impl Header {
    /// Create a new `Header`.
    #[must_use]
    pub fn new(
        destination: Destination,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: Endpoint,
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

    /// Return the cluster type.
    ///
    /// # Errors
    ///
    /// Returns the raw cluster ID if it is unknown.
    pub fn cluster(&self) -> Result<Cluster, u16> {
        self.cluster_id.try_into()
    }

    /// Return the profile ID.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Return the profile type.
    ///
    /// # Errors
    ///
    /// Returns an error if the profile ID is invalid.
    pub fn profile(&self) -> Result<Profile, u16> {
        self.profile_id.try_into()
    }

    /// Return the source endpoint.
    #[must_use]
    pub const fn source_endpoint(&self) -> Endpoint {
        self.source_endpoint
    }

    /// Return the APS frame counter.
    #[must_use]
    pub const fn counter(&self) -> u8 {
        self.counter
    }

    /// Set whether APS security is enabled.
    pub fn set_security(&mut self, enabled: bool) {
        self.control.set(Control::SECURITY, enabled);
    }

    /// Set whether an APS acknowledgement is requested.
    pub fn set_ack_request(&mut self, enabled: bool) {
        self.control.set(Control::ACK_REQUEST, enabled);
    }

    /// Return the extended header.
    #[must_use]
    pub const fn extended(&self) -> Option<Extended> {
        self.extended
    }

    /// Set the extended header and mark it as present in the control field.
    ///
    /// Returns the previously configured extended header, if one was present.
    pub fn set_extended(&mut self, extended: Extended) -> Option<Extended> {
        self.control.insert(Control::EXTENDED_HEADER);
        self.extended.replace(extended)
    }

    /// Drop the extended header.
    pub fn drop_extended(&mut self) -> Option<Extended> {
        self.control.remove(Control::EXTENDED_HEADER);
        self.extended.take()
    }

    /// Set or clear the extended header fragmentation information.
    ///
    /// Passing [`Fragmentation::None`] removes the extended header. Passing a
    /// fragment descriptor installs an extended header that marks this data frame
    /// as the corresponding APS fragment.
    pub fn set_fragmentation(&mut self, fragmentation: Fragmentation) {
        match fragmentation {
            Fragmentation::None => {
                self.drop_extended();
            }
            Fragmentation::First { blocks } => {
                self.set_extended(Extended::first_fragment(blocks));
            }
            Fragmentation::Followup { index } => {
                self.set_extended(Extended::followup_fragment(index));
            }
        }
    }
}

impl FromLeStream for Header {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let control = Control::from_le_stream(&mut bytes)?;
        let destination = control.deserialize_destination(&mut bytes)?;
        let cluster_id = u16::from_le_stream(&mut bytes)?;
        let profile_id = u16::from_le_stream(&mut bytes)?;
        let source_endpoint = Endpoint::from_le_stream(&mut bytes)?;
        let counter = u8::from_le_stream(&mut bytes)?;
        let extended = control.deserialize_extended_header(&mut bytes).ok()?;

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
