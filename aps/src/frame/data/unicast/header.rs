//! Header definitions for an APS data unicast frame.

use le_stream::ToLeStream;

use crate::{Control, DeliveryMode, Extended, FrameType};

/// A header for an APS unicast frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ToLeStream)]
pub struct Header {
    control: Control,
    dst_endpoint: u8,
    cluster_id: u16,
    profile_id: u16,
    source_endpoint: u8,
    counter: u8,
    extended: Option<Extended>,
}

impl Header {
    /// Create a new header for an APS unicast frame.
    #[expect(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        security: bool,
        ack_request: bool,
        dst_endpoint: u8,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        counter: u8,
        extended: Option<Extended>,
    ) -> Self {
        let mut control = Control::empty();
        control.set_frame_type(FrameType::Data);
        control.set_delivery_mode(DeliveryMode::Unicast);
        control.set_extended_header(extended.is_some());

        if security {
            control.insert(Control::SECURITY);
        }

        if ack_request {
            control.insert(Control::ACK_REQUEST);
        }

        Self {
            control,
            dst_endpoint,
            cluster_id,
            profile_id,
            source_endpoint,
            counter,
            extended,
        }
    }

    /// Return the control flags.
    #[must_use]
    pub const fn control(&self) -> Control {
        self.control
    }

    /// Return the destination endpoint ID.
    #[must_use]
    pub const fn dst_endpoint(&self) -> u8 {
        self.dst_endpoint
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

    /// Return the source endpoint ID.
    #[must_use]
    pub const fn source_endpoint(&self) -> u8 {
        self.source_endpoint
    }

    /// Return the APS counter.
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
