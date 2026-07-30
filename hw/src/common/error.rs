use std::fmt::{Display, Formatter};
use std::sync::Arc;

use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::oneshot::error::RecvError;
use zb_aps::apsde::ConfirmStatus;

/// Hardware operation that a backend may not support.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Operation {
    /// Reading local endpoint descriptors.
    GetEndpoints,

    /// Reading the current PAN ID.
    GetPanId,

    /// Reading the coordinator IEEE address.
    GetIeeeAddress,

    /// Scanning for Zigbee networks.
    ScanNetworks,

    /// Scanning channel energy.
    ScanChannels,

    /// Enabling permit joining.
    AllowJoins,

    /// Requesting route discovery.
    RouteRequest,

    /// Resolving a short address to an IEEE address.
    ShortIdToIeeeAddress,

    /// Resolving an IEEE address to a short address.
    IeeeAddressToShortId,

    /// Transmitting an APS frame.
    Transmit,
}

impl Display for Operation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::GetEndpoints => "get endpoints",
            Self::GetPanId => "get PAN ID",
            Self::GetIeeeAddress => "get IEEE address",
            Self::ScanNetworks => "scan networks",
            Self::ScanChannels => "scan channels",
            Self::AllowJoins => "allow joins",
            Self::RouteRequest => "route request",
            Self::ShortIdToIeeeAddress => "short ID to IEEE address translation",
            Self::IeeeAddressToShortId => "IEEE address to short ID translation",
            Self::Transmit => "APS transmission",
        })
    }
}

/// Failure reported for an APS transmission accepted by a hardware backend.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
#[expect(
    variant_size_differences,
    reason = "backend failures retain a shared trait-object source while protocol statuses stay inline"
)]
pub enum TransmissionError {
    /// An accepted APS transmission did not complete before its deadline.
    #[error("APS transmission timed out")]
    Timeout,

    /// No route to the destination was available.
    #[error("No route to APS destination")]
    NoRoute,

    /// The hardware rejected the transmission.
    #[error("APS transmission rejected")]
    Rejected,

    /// APSDE reported an unsuccessful data confirmation.
    #[error("APS data confirmation failed: {0}")]
    Confirmation(ConfirmStatus),

    /// A backend-specific transmission failure occurred.
    #[error("{0}")]
    Backend(#[source] Arc<dyn std::error::Error + Send + Sync>),
}

impl TransmissionError {
    /// Wrap a backend-specific transmission error.
    #[must_use]
    pub fn backend<T>(error: T) -> Self
    where
        T: std::error::Error + Send + Sync + 'static,
    {
        Self::Backend(Arc::new(error))
    }
}

/// A generic error type for Zigbee hardware drivers.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// A backend-specific error occurred.
    #[error("{0}")]
    Backend(#[source] Arc<dyn std::error::Error + Send + Sync>),

    /// The driver actor is unavailable.
    #[error("Driver actor unavailable")]
    ActorUnavailable,

    /// The backend does not support the requested operation.
    #[error("Unsupported hardware operation: {0}")]
    Unsupported(Operation),

    /// An accepted APS transmission failed.
    #[error(transparent)]
    Transmission(#[from] TransmissionError),
}

impl Error {
    /// Wrap a backend-specific hardware error.
    #[must_use]
    pub fn backend<T>(error: T) -> Self
    where
        T: std::error::Error + Send + Sync + 'static,
    {
        Self::Backend(Arc::new(error))
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_: SendError<T>) -> Self {
        Self::ActorUnavailable
    }
}

impl From<RecvError> for Error {
    fn from(_: RecvError) -> Self {
        Self::ActorUnavailable
    }
}
