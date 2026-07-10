use apis_saltans_core::Profile;
use bytes::Bytes;

/// Serialized application payload plus APS metadata for transmission.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Datagram {
    metadata: Metadata,
    payload: Bytes,
}

impl Datagram {
    /// Construct a datagram from pre-serialized bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `payload` is a valid serialized application payload for the
    /// supplied APS `metadata`. The hardware layer does not parse or validate this relationship.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(metadata: Metadata, payload: Bytes) -> Self {
        Self { metadata, payload }
    }

    /// Return the APS metadata and serialized payload, consuming the datagram.
    #[must_use]
    pub fn into_parts(self) -> (Metadata, Bytes) {
        (self.metadata, self.payload)
    }
}

/// APS metadata associated with a serialized application payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Metadata {
    profile: Profile,
    cluster_id: u16,
}

impl Metadata {
    /// Create metadata for an APS profile and cluster.
    #[must_use]
    pub const fn new(profile: Profile, cluster_id: u16) -> Self {
        Self {
            profile,
            cluster_id,
        }
    }

    /// Return the APS profile.
    #[must_use]
    pub const fn profile(self) -> Profile {
        self.profile
    }

    /// Return the APS cluster ID.
    #[must_use]
    pub const fn cluster_id(self) -> u16 {
        self.cluster_id
    }
}
