use le_stream::{FromLeStream, ToLeStream};

/// Additional ack aps format information.
///
/// This structure is present in acknowledgment frames when the `ack format` bit
/// in the control field is *not* set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct AckFmt {
    destination: u8,
    cluster_id: u16,
    profile_id: u16,
    source: u8,
}

impl AckFmt {
    /// Creates a new `AckFmt`.
    #[must_use]
    pub const fn new(destination: u8, cluster_id: u16, profile_id: u16, source: u8) -> Self {
        Self {
            destination,
            cluster_id,
            profile_id,
            source,
        }
    }

    /// Returns the destination endpoint.
    #[must_use]
    pub const fn destination(&self) -> u8 {
        self.destination
    }

    /// Returns the cluster ID.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Returns the profile ID.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Returns the source endpoint.
    #[must_use]
    pub const fn source(&self) -> u8 {
        self.source
    }
}
