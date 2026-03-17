//! Header definitions for an APS data unicast frame.

use crate::{Control, Extended};

/// A header for an APS unicast frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
    ///
    /// # Side effects
    ///
    /// This will override the control flag for the extended header,
    /// depending on whether an extended header is set or not.
    #[must_use]
    pub fn new(
        mut control: Control,
        dst_endpoint: u8,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        counter: u8,
        extended: Option<Extended>,
    ) -> Self {
        // Ensure that we have the correct flag set in either case.
        if extended.is_some() {
            control.insert(Control::EXTENDED_HEADER);
        } else {
            control.remove(Control::EXTENDED_HEADER);
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

    /// Create a new header for an APS unicast frame.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the control field correctly indicates whether there's an extended header.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(
        control: Control,
        dst_endpoint: u8,
        cluster_id: u16,
        profile_id: u16,
        source_endpoint: u8,
        counter: u8,
        extended: Option<Extended>,
    ) -> Self {
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
