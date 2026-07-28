use zb_core::{Cluster, Profile};

use super::{Alias, IndividualEndpoint, RequestDestination, TxOptions};

const DEFAULT_RADIUS_COUNTER: u8 = 0;

/// Parameters of an `APSDE-DATA.request` primitive.
///
/// The ASDU is generic so callers can use owned bytes, borrowed byte slices,
/// or a higher-level payload type. The ASDU length is derived from the payload
/// instead of being stored independently.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataRequest<T> {
    destination: RequestDestination,
    profile_id: u16,
    cluster_id: u16,
    source_endpoint: IndividualEndpoint,
    asdu: T,
    tx_options: TxOptions,
    alias: Alias,
    radius_counter: u8,
}

impl<T> DataRequest<T> {
    /// Create a data request without optional transmission behaviors.
    #[must_use]
    pub const fn new(
        destination: RequestDestination,
        profile_id: u16,
        cluster_id: u16,
        source_endpoint: IndividualEndpoint,
        asdu: T,
    ) -> Self {
        Self {
            destination,
            profile_id,
            cluster_id,
            source_endpoint,
            asdu,
            tx_options: TxOptions::empty(),
            alias: Alias::None,
            radius_counter: DEFAULT_RADIUS_COUNTER,
        }
    }

    /// Set the APS transmission options.
    #[must_use]
    pub const fn with_tx_options(mut self, tx_options: TxOptions) -> Self {
        self.tx_options = tx_options;
        self
    }

    /// Set the NWK source-alias parameters.
    #[must_use]
    pub const fn with_alias(mut self, alias: Alias) -> Self {
        self.alias = alias;
        self
    }

    /// Set the maximum number of network hops.
    ///
    /// Zero delegates radius selection to the NWK layer.
    #[must_use]
    pub const fn with_radius_counter(mut self, radius_counter: u8) -> Self {
        self.radius_counter = radius_counter;
        self
    }

    /// Return the requested destination.
    #[must_use]
    pub const fn destination(&self) -> RequestDestination {
        self.destination
    }

    /// Return the raw application profile identifier.
    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    /// Interpret the application profile identifier.
    ///
    /// # Errors
    ///
    /// Returns the unchanged raw identifier when it is not a profile known by
    /// `apis-saltans-core`.
    pub fn profile(&self) -> Result<Profile, u16> {
        self.profile_id.try_into()
    }

    /// Return the raw cluster identifier.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        self.cluster_id
    }

    /// Interpret the cluster identifier.
    ///
    /// # Errors
    ///
    /// Returns the unchanged raw identifier when it is not a cluster known by
    /// `apis-saltans-core`.
    pub fn cluster(&self) -> Result<Cluster, u16> {
        self.cluster_id.try_into()
    }

    /// Return the local source endpoint.
    #[must_use]
    pub const fn source_endpoint(&self) -> IndividualEndpoint {
        self.source_endpoint
    }

    /// Return the application-service data unit.
    #[must_use]
    pub const fn asdu(&self) -> &T {
        &self.asdu
    }

    /// Consume the request and return the application-service data unit.
    #[must_use]
    pub fn into_asdu(self) -> T {
        self.asdu
    }

    /// Transform the ASDU while preserving all request metadata.
    #[must_use]
    pub fn map_asdu<U, F>(self, map: F) -> DataRequest<U>
    where
        F: FnOnce(T) -> U,
    {
        DataRequest {
            destination: self.destination,
            profile_id: self.profile_id,
            cluster_id: self.cluster_id,
            source_endpoint: self.source_endpoint,
            asdu: map(self.asdu),
            tx_options: self.tx_options,
            alias: self.alias,
            radius_counter: self.radius_counter,
        }
    }

    /// Return the APS transmission options.
    #[must_use]
    pub const fn tx_options(&self) -> TxOptions {
        self.tx_options
    }

    /// Return the NWK source-alias parameters.
    #[must_use]
    pub const fn alias(&self) -> Alias {
        self.alias
    }

    /// Return the maximum network-hop count.
    #[must_use]
    pub const fn radius_counter(&self) -> u8 {
        self.radius_counter
    }
}

impl<T> DataRequest<T>
where
    T: AsRef<[u8]>,
{
    /// Return the ASDU length in octets.
    #[must_use]
    pub fn asdu_length(&self) -> usize {
        self.asdu.as_ref().len()
    }
}

#[cfg(test)]
mod tests {
    use zb_core::{Endpoint, Profile};

    use super::super::{Alias, IndividualEndpoint, NetworkAddress, RequestDestination, TxOptions};
    use super::DataRequest;

    const ALIAS_SEQUENCE_NUMBER: u8 = 7;
    const ASDU: [u8; 3] = [1, 2, 3];
    const CLUSTER_ID: u16 = 0x0006;
    const DESTINATION_ADDRESS: u16 = 0x1234;
    const RADIUS_COUNTER: u8 = 5;

    #[test]
    fn request_derives_length_and_preserves_optional_parameters() {
        let endpoint =
            IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
        let address = NetworkAddress::new(DESTINATION_ADDRESS).expect("test NWK address is valid");
        let destination = RequestDestination::Network {
            address,
            endpoint: Endpoint::Broadcast,
        };
        let request = DataRequest::new(
            destination,
            Profile::ZigbeeHomeAutomation.as_u16(),
            CLUSTER_ID,
            endpoint,
            ASDU,
        )
        .with_tx_options(TxOptions::ACKNOWLEDGED_TRANSMISSION)
        .with_alias(Alias::Use {
            source: address,
            sequence_number: ALIAS_SEQUENCE_NUMBER,
        })
        .with_radius_counter(RADIUS_COUNTER);

        assert_eq!(request.asdu_length(), ASDU.len());
        assert_eq!(request.destination(), destination);
        assert_eq!(request.tx_options(), TxOptions::ACKNOWLEDGED_TRANSMISSION);
        assert!(request.alias().is_used());
        assert_eq!(request.radius_counter(), RADIUS_COUNTER);
    }
}
