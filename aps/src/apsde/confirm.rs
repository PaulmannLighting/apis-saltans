use core::fmt::{self, Display, Formatter};

use zb_core::{AcknowledgementLayer, NetworkError};

use super::{Destination, IndividualEndpoint, Status};

/// Status reported by an `APSDE-DATA.confirm`.
///
/// APSDE may return a native APS status or a backend-reported networking failure.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConfirmStatus {
    /// APS-layer completion status.
    Aps(Status),

    /// Zigbee networking failure reported by the backend.
    Failure(NetworkError),
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
    /// Return the networking failure, if present or recognized from an APS status.
    ///
    /// Unclassified backend bytes are returned as `NetworkError::BackendStatus`.
    #[must_use]
    pub const fn network_error(self) -> Option<NetworkError> {
        match self {
            Self::Failure(error) => Some(error),
            Self::Aps(Status::NoAcknowledgement) => Some(NetworkError::NoAcknowledgement {
                layer: AcknowledgementLayer::Aps,
            }),
            Self::Aps(Status::AsduTooLong) => Some(NetworkError::MessageTooLong),
            Self::Aps(Status::NoShortAddress) => Some(NetworkError::AddressResolutionFailed),
            Self::Aps(Status::SecurityFailure) => Some(NetworkError::SecurityFailure),
            Self::Aps(Status::TableFull) => Some(NetworkError::TableFull),
            Self::Aps(_) => None,
        }
    }

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
            Self::Failure(error) => Display::fmt(error, formatter),
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
    use super::{ConfirmStatus, DataConfirm, NetworkError};

    const BACKEND_STATUS: u8 = 0xc1;
    const TX_TIME: u64 = 42;

    #[test]
    fn confirmation_preserves_an_unclassified_backend_status() {
        let source_endpoint =
            IndividualEndpoint::new(Endpoint::Data).expect("data endpoint is individual");
        let confirmation = DataConfirm::new(
            Destination::Bound,
            source_endpoint,
            ConfirmStatus::Failure(NetworkError::BackendStatus(BACKEND_STATUS)),
            TX_TIME,
        );

        assert_eq!(confirmation.destination(), Destination::Bound);
        assert_eq!(confirmation.source_endpoint(), source_endpoint);
        assert_eq!(
            confirmation.status(),
            ConfirmStatus::Failure(NetworkError::BackendStatus(BACKEND_STATUS))
        );
        assert_eq!(confirmation.tx_time(), &TX_TIME);
    }
}

#[cfg(test)]
mod classification_tests {
    use super::{AcknowledgementLayer, ConfirmStatus, NetworkError, Status};

    #[test]
    fn failures_are_inspectable_without_guessing_backend_status_semantics() {
        assert_eq!(
            ConfirmStatus::Aps(Status::NoAcknowledgement).network_error(),
            Some(NetworkError::NoAcknowledgement {
                layer: AcknowledgementLayer::Aps
            })
        );
        assert_eq!(
            ConfirmStatus::Aps(Status::SecurityFailure).network_error(),
            Some(NetworkError::SecurityFailure)
        );
        assert_eq!(ConfirmStatus::success().network_error(), None);
        for raw in u8::MIN..=u8::MAX {
            let failure = ConfirmStatus::Failure(NetworkError::BackendStatus(raw));
            assert_eq!(
                failure.network_error(),
                Some(NetworkError::BackendStatus(raw))
            );
            assert!(!failure.is_success());
        }
    }
}
