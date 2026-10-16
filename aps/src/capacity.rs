#[cfg(not(feature = "alloc"))]
use const_env::env_item;
use thiserror::Error;

/// Maximum owned or reassembled payload size without `alloc`, in bytes.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_APS_PAYLOAD_CAPACITY")]
pub const PAYLOAD_CAPACITY: usize = 1024;

/// Maximum buffered fragment payload size without `alloc`, in bytes.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_APS_FRAGMENT_CAPACITY")]
pub const FRAGMENT_CAPACITY: usize = 128;

/// Maximum fragments in one transaction without `alloc`.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_APS_FRAGMENTS_CAPACITY")]
pub const FRAGMENTS_CAPACITY: usize = 16;

/// Maximum simultaneous reassembly transactions without `alloc`.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_APS_TRANSACTIONS_CAPACITY")]
pub const TRANSACTIONS_CAPACITY: usize = 4;

#[cfg(not(feature = "alloc"))]
const _: () = {
    assert!(
        PAYLOAD_CAPACITY > 0,
        "APS payload capacity must be positive"
    );
    assert!(
        FRAGMENT_CAPACITY > 0,
        "APS fragment capacity must be positive"
    );
    assert!(
        FRAGMENT_CAPACITY <= PAYLOAD_CAPACITY,
        "APS fragment capacity must not exceed payload capacity"
    );
    assert!(
        FRAGMENTS_CAPACITY > 0 && FRAGMENTS_CAPACITY <= u8::MAX as usize,
        "APS fragment count capacity must be in 1..=255"
    );
    assert!(
        TRANSACTIONS_CAPACITY > 0,
        "APS transaction capacity must be positive"
    );
};

/// Bounded resource that could not accommodate an operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CapacityKind {
    /// Owned or reassembled bytes.
    Payload,

    /// Bytes in one buffered fragment.
    Fragment,

    /// Fragment slots in one transaction.
    Fragments,

    /// Simultaneous transactions.
    Transactions,
}

/// A configured allocation-free capacity was exceeded.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("APS {kind:?} capacity exceeded: requires at least {required}, capacity is {capacity}")]
pub struct CapacityError {
    /// Resource that was exhausted.
    pub kind: CapacityKind,
    /// Required size, or the first excessive size encountered when reading an iterator.
    pub required: usize,
    /// Configured maximum size.
    pub capacity: usize,
}
