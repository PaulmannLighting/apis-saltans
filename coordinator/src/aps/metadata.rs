use zb_aps::TxOptions;
use zb_core::Profile;

const DEFAULT_TX_OPTIONS: TxOptions = TxOptions::ACKNOWLEDGED_TRANSMISSION;

/// APS fields used to construct a ZDP data request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Metadata {
    profile: Profile,
    cluster_id: u16,
    tx_options: TxOptions,
}

impl Metadata {
    /// Create APS metadata for a profile and cluster.
    ///
    /// Transmissions request an APS acknowledgement by default. ZDP supplies its fixed data
    /// endpoint when it constructs the complete request.
    #[must_use]
    pub const fn new(profile: Profile, cluster_id: u16) -> Self {
        Self {
            profile,
            cluster_id,
            tx_options: DEFAULT_TX_OPTIONS,
        }
    }

    /// Return the application profile.
    #[must_use]
    pub const fn profile(self) -> Profile {
        self.profile
    }

    /// Return the cluster identifier.
    #[must_use]
    pub const fn cluster_id(self) -> u16 {
        self.cluster_id
    }

    /// Return the APS transmission options.
    #[must_use]
    pub const fn tx_options(self) -> TxOptions {
        self.tx_options
    }
}

#[cfg(test)]
mod tests {
    use zb_aps::TxOptions;
    use zb_core::Profile;

    use super::Metadata;

    const CLUSTER_ID: u16 = 0x1234;

    #[test]
    fn metadata_requests_acknowledgement_by_default() {
        let metadata = Metadata::new(Profile::ZigbeeHomeAutomation, CLUSTER_ID);

        assert!(
            metadata
                .tx_options()
                .contains(TxOptions::ACKNOWLEDGED_TRANSMISSION)
        );
    }
}
