use core::fmt::{self, Display, Formatter};

use super::{Destination, IndividualEndpoint, Status};

/// Status reported by an `APSDE-DATA.confirm`.
///
/// APSDE may return either a native APS status or a status propagated from the
/// implementation's network data entity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfirmStatus {
    /// APS-layer completion status.
    Aps(Status),

    /// Status propagated from `NLDE-DATA.confirm`.
    Network(u8),
}

/// Parameters of an `APSDE-DATA.confirm` primitive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataConfirm<T> {
    destination: Destination,
    source_endpoint: IndividualEndpoint,
    status: ConfirmStatus,
    tx_time: T,
}

impl ConfirmStatus {
    /// Return a successful APS completion status.
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

impl From<Status> for ConfirmStatus {
    fn from(status: Status) -> Self {
        Self::Aps(status)
    }
}

impl Display for ConfirmStatus {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Aps(status) => Display::fmt(status, formatter),
            Self::Network(status) => write!(formatter, "NWK status {status:#04x}"),
        }
    }
}

impl<T> DataConfirm<T> {
    /// Create a data confirmation.
    #[must_use]
    pub const fn new(
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        status: ConfirmStatus,
        tx_time: T,
    ) -> Self {
        Self {
            destination,
            source_endpoint,
            status,
            tx_time,
        }
    }

    /// Return the confirmed destination.
    #[must_use]
    pub const fn destination(&self) -> Destination {
        self.destination
    }

    /// Return the local source endpoint.
    #[must_use]
    pub const fn source_endpoint(&self) -> IndividualEndpoint {
        self.source_endpoint
    }

    /// Return the transmission status.
    #[must_use]
    pub const fn status(&self) -> ConfirmStatus {
        self.status
    }

    /// Return the implementation-specific transmission timestamp.
    #[must_use]
    pub const fn tx_time(&self) -> &T {
        &self.tx_time
    }

    /// Consume the confirmation and return its transmission timestamp.
    #[must_use]
    pub fn into_tx_time(self) -> T {
        self.tx_time
    }
}

#[cfg(test)]
mod tests {
    use zb_core::Endpoint;

    use super::super::{Destination, IndividualEndpoint};
    use super::{ConfirmStatus, DataConfirm};

    const NETWORK_STATUS: u8 = 0xc1;
    const TX_TIME: u64 = 42;

    #[test]
    fn confirmation_preserves_a_propagated_network_status() {
        let source_endpoint =
            IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
        let confirmation = DataConfirm::new(
            Destination::Bound,
            source_endpoint,
            ConfirmStatus::Network(NETWORK_STATUS),
            TX_TIME,
        );

        assert_eq!(confirmation.destination(), Destination::Bound);
        assert_eq!(confirmation.source_endpoint(), source_endpoint);
        assert_eq!(
            confirmation.status(),
            ConfirmStatus::Network(NETWORK_STATUS)
        );
        assert_eq!(confirmation.tx_time(), &TX_TIME);
    }
}
