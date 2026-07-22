use bytes::Bytes;
use zb_aps::TxOptions;
use zb_core::Profile;

const DEFAULT_TX_OPTIONS: TxOptions = TxOptions::ACKNOWLEDGED_TRANSMISSION;

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

/// APS metadata and transmission options associated with a serialized application payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Metadata {
    profile: Profile,
    cluster_id: u16,
    tx_options: TxOptions,
}

impl Metadata {
    /// Create metadata for an APS profile and cluster with acknowledged transmission enabled.
    #[must_use]
    pub const fn new(profile: Profile, cluster_id: u16) -> Self {
        Self {
            profile,
            cluster_id,
            tx_options: DEFAULT_TX_OPTIONS,
        }
    }

    /// Override the APSDE-DATA transmission options.
    #[must_use]
    pub const fn with_tx_options(mut self, tx_options: TxOptions) -> Self {
        self.tx_options = tx_options;
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

    /// Return the APSDE-DATA transmission options.
    #[must_use]
    pub const fn tx_options(self) -> TxOptions {
        self.tx_options
    }
}

#[cfg(test)]
mod tests {
    use zb_aps::TxOptions;
    use zb_core::{Cluster, Profile};

    use super::Metadata;

    const PROFILE: Profile = Profile::ZigbeeHomeAutomation;
    const CLUSTER_ID: u16 = Cluster::OtaUpgrade.as_u16();

    #[test]
    fn requests_acknowledged_transmission_by_default() {
        let metadata = Metadata::new(PROFILE, CLUSTER_ID);

        assert_eq!(metadata.tx_options(), TxOptions::ACKNOWLEDGED_TRANSMISSION);
    }

    #[test]
    fn overrides_all_transmission_options() {
        let options = TxOptions::SECURITY_ENABLED | TxOptions::FRAGMENTATION_PERMITTED;
        let metadata = Metadata::new(PROFILE, CLUSTER_ID).with_tx_options(options);

        assert_eq!(metadata.tx_options(), options);
    }
}
