use zb_core::{Cluster, Profile};

use super::{ReceivedDestination, Security, Source, Status};

/// Incoming-frame processing status reported by `APSDE-DATA.indication`.
///
/// APSDE may report either a native APS status or a status propagated from the
/// implementation's security processing.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IndicationStatus {
    /// APS-layer incoming-frame status.
    Aps(Status),

    /// Status propagated from security processing.
    Security(u8),
}

/// Metadata of an `APSDE-DATA.indication` primitive.
///
/// The timestamp and device-key-pair handle are implementation-defined and
/// therefore generic.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IndicationMetadata<T, K = ()> {
    destination: ReceivedDestination,
    source: Source,
    profile_id: u16,
    cluster_id: u16,
    status: IndicationStatus,
    security: Security<K>,
    link_quality: u8,
    rx_time: T,
}

/// An ASDU and its `APSDE-DATA.indication` metadata.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataIndication<A, T, K = ()> {
    metadata: IndicationMetadata<T, K>,
    asdu: A,
}

impl IndicationStatus {
    /// Return a successful APS incoming-frame status.
    #[must_use]
    pub const fn success() -> Self {
        Self::Aps(Status::Success)
    }

    /// Return whether this is an APS success status.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Aps(Status::Success))
    }
}

impl From<Status> for IndicationStatus {
    fn from(status: Status) -> Self {
        Self::Aps(status)
    }
}

impl<T, K> IndicationMetadata<T, K> {
    /// Create received APSDE metadata.
    #[expect(
        clippy::too_many_arguments,
        reason = "fields mirror the APSDE primitive"
    )]
    #[must_use]
    pub const fn new(
        destination: ReceivedDestination,
        source: Source,
        profile_id: u16,
        cluster_id: u16,
        status: IndicationStatus,
        security: Security<K>,
        link_quality: u8,
        rx_time: T,
    ) -> Self {
        Self {
            destination,
            source,
            profile_id,
            cluster_id,
            status,
            security,
            link_quality,
            rx_time,
        }
    }

    /// Return the received destination.
    #[must_use]
    pub const fn destination(&self) -> ReceivedDestination {
        self.destination
    }

    /// Return the received source.
    #[must_use]
    pub const fn source(&self) -> Source {
        self.source
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

    /// Return the incoming-frame processing status.
    #[must_use]
    pub const fn status(&self) -> IndicationStatus {
        self.status
    }

    /// Return the incoming security metadata.
    #[must_use]
    pub const fn security(&self) -> &Security<K> {
        &self.security
    }

    /// Return the link-quality indication delivered by NWK.
    #[must_use]
    pub const fn link_quality(&self) -> u8 {
        self.link_quality
    }

    /// Return the implementation-specific reception timestamp.
    #[must_use]
    pub const fn rx_time(&self) -> &T {
        &self.rx_time
    }

    /// Transform the implementation-defined timestamp and device-key-pair handle.
    #[must_use]
    pub fn map_context<U, L, F, G>(self, map_time: F, map_key_pair: G) -> IndicationMetadata<U, L>
    where
        F: FnOnce(T) -> U,
        G: FnOnce(K) -> L,
    {
        IndicationMetadata {
            destination: self.destination,
            source: self.source,
            profile_id: self.profile_id,
            cluster_id: self.cluster_id,
            status: self.status,
            security: self.security.map_key_pair(map_key_pair),
            link_quality: self.link_quality,
            rx_time: map_time(self.rx_time),
        }
    }
}

impl<A, T, K> DataIndication<A, T, K> {
    /// Attach an ASDU to received APSDE metadata.
    #[must_use]
    pub const fn new(metadata: IndicationMetadata<T, K>, asdu: A) -> Self {
        Self { metadata, asdu }
    }

    /// Return the indication metadata.
    #[must_use]
    pub const fn metadata(&self) -> &IndicationMetadata<T, K> {
        &self.metadata
    }

    /// Return the application-service data unit.
    #[must_use]
    pub const fn asdu(&self) -> &A {
        &self.asdu
    }

    /// Consume the indication and return its metadata and ASDU.
    #[must_use]
    pub fn into_parts(self) -> (IndicationMetadata<T, K>, A) {
        (self.metadata, self.asdu)
    }

    /// Transform the ASDU while preserving all indication metadata.
    #[must_use]
    pub fn map_asdu<B, F>(self, map: F) -> DataIndication<B, T, K>
    where
        F: FnOnce(A) -> B,
    {
        DataIndication {
            metadata: self.metadata,
            asdu: map(self.asdu),
        }
    }

    /// Transform the implementation-defined timestamp and device-key-pair handle.
    #[must_use]
    pub fn map_context<U, L, F, G>(self, map_time: F, map_key_pair: G) -> DataIndication<A, U, L>
    where
        F: FnOnce(T) -> U,
        G: FnOnce(K) -> L,
    {
        DataIndication {
            metadata: self.metadata.map_context(map_time, map_key_pair),
            asdu: self.asdu,
        }
    }
}

impl<A, T, K> DataIndication<A, T, K>
where
    A: AsRef<[u8]>,
{
    /// Return the ASDU length in octets.
    #[must_use]
    pub fn asdu_length(&self) -> usize {
        self.asdu.as_ref().len()
    }
}

#[cfg(test)]
mod tests {
    use zb_core::{Endpoint, GroupId, Profile};

    use super::super::{
        IndividualEndpoint, NetworkAddress, ReceivedDestination, Security, Source, Status,
    };
    use super::{DataIndication, IndicationMetadata, IndicationStatus};

    const ASDU: [u8; 3] = [1, 2, 3];
    const CLUSTER_ID: u16 = 0x0006;
    const DEVICE_KEY_PAIR_ENTRY: usize = 7;
    const GROUP_ID: u16 = 0x1234;
    const KEY_INDEX: u8 = 2;
    const LINK_QUALITY: u8 = u8::MAX;
    const RX_TIME: u64 = 42;
    const SOURCE_ADDRESS: u16 = 0x4321;

    #[test]
    fn indication_keeps_transport_and_security_metadata_with_the_asdu() {
        let endpoint =
            IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
        let metadata = IndicationMetadata::new(
            ReceivedDestination::Group(
                GroupId::new(GROUP_ID).expect("test group identifier is valid"),
            ),
            Source::Network {
                address: NetworkAddress::new(SOURCE_ADDRESS).expect("test source address is valid"),
                endpoint,
            },
            Profile::ZigbeeHomeAutomation.as_u16(),
            CLUSTER_ID,
            IndicationStatus::Aps(Status::Success),
            Security::LinkKey {
                key_index: KEY_INDEX,
                device_key_pair_entry: DEVICE_KEY_PAIR_ENTRY,
            },
            LINK_QUALITY,
            RX_TIME,
        );
        let indication = DataIndication::new(metadata, ASDU);

        assert_eq!(indication.asdu_length(), ASDU.len());
        assert_eq!(
            indication.metadata().status(),
            IndicationStatus::Aps(Status::Success)
        );
        assert!(matches!(
            indication.metadata().security(),
            Security::LinkKey {
                key_index: KEY_INDEX,
                device_key_pair_entry: DEVICE_KEY_PAIR_ENTRY
            }
        ));

        let normalized = indication.map_context(drop, drop);
        assert_eq!(normalized.metadata().rx_time(), &());
        assert!(matches!(
            normalized.metadata().security(),
            Security::LinkKey {
                key_index: KEY_INDEX,
                device_key_pair_entry: ()
            }
        ));
        assert_eq!(normalized.asdu(), &ASDU);
    }
}
