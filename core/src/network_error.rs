use thiserror::Error;

/// Protocol layer whose acknowledgement was missing.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum AcknowledgementLayer {
    /// End-to-end APS acknowledgement.
    Aps,

    /// Single-hop MAC acknowledgement.
    Mac,
}

/// Zigbee networking failure across APS, NWK, MAC, and local stack operations.
///
/// Named variants provide portable classifications; `BackendStatus` retains a code
/// without assigning portable semantics. This is not a guarantee that retrying is safe
/// or will succeed.
#[derive(Clone, Copy, Debug, Eq, Error, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum NetworkError {
    /// Delivery failed; the backend did not identify a more specific cause.
    #[error("Delivery failed; the backend did not identify a more specific cause.")]
    DeliveryFailed,

    /// An expected acknowledgement was not received.
    #[error("An expected {layer:?} acknowledgement was not received.")]
    NoAcknowledgement {
        /// Layer at which acknowledgement was expected.
        layer: AcknowledgementLayer,
    },

    /// No usable route to the destination exists.
    #[error("No usable route to the destination exists.")]
    NoRoute,

    /// A source route failed.
    #[error("A source route failed.")]
    SourceRouteFailure,

    /// A many-to-one route failed.
    #[error("A many-to-one route failed.")]
    ManyToOneRouteFailure,

    /// Clear-channel assessment exhausted its attempts to access the radio channel.
    #[error("Clear-channel assessment exhausted its attempts to access the radio channel.")]
    ChannelAccessFailure,

    /// The network is overloaded.
    #[error("The network is overloaded.")]
    NetworkBusy,

    /// Local buffers, message slots, or queue capacity are exhausted.
    #[error("Local buffers, message slots, or queue capacity are exhausted.")]
    ResourcesExhausted,

    /// The local network is not operating.
    #[error("The local network is not operating.")]
    NetworkDown,

    /// The local node has not joined a network.
    #[error("The local node has not joined a network.")]
    NotJoined,

    /// The destination could not be resolved to a network address.
    #[error("The destination could not be resolved to a network address.")]
    AddressResolutionFailed,

    /// The message exceeds the supported transmission size.
    #[error("The message exceeds the supported transmission size.")]
    MessageTooLong,

    /// Security processing failed; the backend source may provide further details.
    #[error("Security processing failed; the backend source may provide further details.")]
    SecurityFailure,

    /// An indirect message expired before the child polled for it.
    #[error("An indirect message expired before the child polled for it.")]
    IndirectTimeout,

    /// A required management table has no free entry.
    #[error("A required management table has no free entry.")]
    TableFull,

    /// Backend failure status without a portable classification.
    ///
    /// The code may be recognized by the backend. Interpret it only in that backend's
    /// status namespace, never as an assumed Zigbee NWK status.
    #[error("Backend status {0:#04x}")]
    BackendStatus(u8),
}
