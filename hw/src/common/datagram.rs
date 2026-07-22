use bytes::Bytes;
use zb_core::Profile;

const DEFAULT_APS_ACKNOWLEDGEMENT: bool = true;

/// Serialized application payload plus APS metadata for transmission.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Datagram {
    metadata: Metadata,
    payload: Bytes,
}

impl Datagram {
    /// Construct a datagram from APS metadata and a pre-serialized application payload.
    #[must_use]
    pub const fn new(metadata: Metadata, payload: Bytes) -> Self {
        Self { metadata, payload }
    }

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
    aps_acknowledgement: bool,
}

impl Metadata {
    /// Create metadata for an APS profile and cluster.
    #[must_use]
    pub const fn new(profile: Profile, cluster_id: u16) -> Self {
        Self {
            profile,
            cluster_id,
            aps_acknowledgement: DEFAULT_APS_ACKNOWLEDGEMENT,
        }
    }

    /// Override whether APS acknowledgement and retries are requested for this transmission.
    #[must_use]
    pub const fn with_aps_acknowledgement(mut self, enabled: bool) -> Self {
        self.aps_acknowledgement = enabled;
        self
    }

    /// Override the APS application profile while preserving transmission options.
    #[must_use]
    pub const fn with_profile(mut self, profile: Profile) -> Self {
        self.profile = profile;
        self
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

    /// Return whether the driver should request APS acknowledgement and retries.
    #[must_use]
    pub const fn aps_acknowledgement(self) -> bool {
        self.aps_acknowledgement
    }
}
