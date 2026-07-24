use zb_aps::TxOptions;
use zb_core::{Endpoint, Profile};

const DEFAULT_TX_OPTIONS: TxOptions = TxOptions::ACKNOWLEDGED_TRANSMISSION;

/// Metadata used to construct an outgoing APS data frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Metadata {
    profile: Profile,
    cluster_id: u16,
    source_endpoint: Endpoint,
    tx_options: TxOptions,
}

impl Metadata {
    /// Create APS metadata for a profile and cluster.
    ///
    /// Network-profile commands use the ZDO data endpoint. Application-profile commands use the
    /// first application endpoint. Transmissions request an APS acknowledgement by default.
    #[must_use]
    pub const fn new(profile: Profile, cluster_id: u16) -> Self {
        let source_endpoint = if matches!(profile, Profile::Network) {
            Endpoint::Data
        } else {
            Endpoint::Application(zb_core::endpoint::Application::MIN)
        };

        Self {
            profile,
            cluster_id,
            source_endpoint,
            tx_options: DEFAULT_TX_OPTIONS,
        }
    }

    /// Override the source endpoint.
    #[must_use]
    pub const fn with_source_endpoint(mut self, source_endpoint: Endpoint) -> Self {
        self.source_endpoint = source_endpoint;
        self
    }

    /// Override the APS transmission options.
    #[must_use]
    pub const fn with_tx_options(mut self, tx_options: TxOptions) -> Self {
        self.tx_options = tx_options;
        self
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

    /// Return the source endpoint.
    #[must_use]
    pub const fn source_endpoint(self) -> Endpoint {
        self.source_endpoint
    }

    /// Return the APS transmission options.
    #[must_use]
    pub const fn tx_options(self) -> TxOptions {
        self.tx_options
    }

    /// Return whether the transmission requests an APS acknowledgement.
    #[must_use]
    pub const fn acknowledged(self) -> bool {
        self.tx_options
            .contains(TxOptions::ACKNOWLEDGED_TRANSMISSION)
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

        assert!(metadata.acknowledged());
    }

    #[test]
    fn empty_options_disable_acknowledgement() {
        let metadata = Metadata::new(Profile::ZigbeeHomeAutomation, CLUSTER_ID)
            .with_tx_options(TxOptions::empty());

        assert!(!metadata.acknowledged());
    }
}
