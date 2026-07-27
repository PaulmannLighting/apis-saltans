use zb_aps::TxOptions;
use zb_core::{Destination, Endpoint, Profile};

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

    /// Override the application profile.
    #[must_use]
    pub const fn with_profile(mut self, profile: Profile) -> Self {
        self.profile = profile;
        self
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

    /// Return whether this transmission requests an acknowledgement for the destination.
    ///
    /// APS acknowledgements apply only to unicast device transmissions. Group and broadcast
    /// destinations ignore the acknowledgement option.
    #[must_use]
    pub const fn acknowledged_for(self, destination: Destination) -> bool {
        self.acknowledged() && matches!(destination, Destination::Device(_))
    }
}

#[cfg(test)]
mod tests {
    use zb_aps::TxOptions;
    use zb_core::destination::{Broadcast, Device};
    use zb_core::endpoint::Application;
    use zb_core::{Destination, Endpoint, GroupId, Profile, short_id};

    use super::Metadata;

    const CLUSTER_ID: u16 = 0x1234;
    const DEVICE_ID: u16 = 0x1234;
    const GROUP_ID: u16 = 0x2345;

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

    #[test]
    fn acknowledgement_applies_only_to_unicast_destinations() {
        let metadata = Metadata::new(Profile::ZigbeeHomeAutomation, CLUSTER_ID);
        let endpoint = Endpoint::Application(Application::MIN);
        let device = short_id::Device::new(DEVICE_ID).expect("test device ID is valid");
        let group = GroupId::new(GROUP_ID).expect("test group ID is valid");

        assert!(metadata.acknowledged_for(Destination::Device(Device::new(device, endpoint))));
        assert!(!metadata.acknowledged_for(Destination::Group(group)));
        assert!(
            !metadata.acknowledged_for(Destination::Broadcast(Broadcast::new(
                short_id::Broadcast::AllDevices,
                Endpoint::Broadcast,
            )))
        );
    }
}
