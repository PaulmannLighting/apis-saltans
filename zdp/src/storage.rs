//! Storage selected by the `alloc` feature.

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, vec, vec::Vec};

#[cfg(feature = "alloc")]
use const_env as _;
#[cfg(not(feature = "alloc"))]
use const_env::env_item;
use zb_core::types::tlv::Tlv;

#[cfg(not(feature = "alloc"))]
pub use self::bounded::{RawList, Stream};

#[cfg(not(feature = "alloc"))]
mod bounded;
#[cfg(test)]
mod tests;

#[cfg(not(feature = "alloc"))]
pub(crate) const PREFIX_CAPACITY: usize = 16;

/// Maximum TLVs per command without allocation.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZDP_TLV_CAPACITY")]
pub const TLV_CAPACITY: usize = 2;
/// Capacity for associated addresses without allocation; must accommodate a full byte-sized list.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZDP_ADDRESS_CAPACITY")]
pub const ADDRESS_CAPACITY: usize = 255;
/// Capacity for returned cluster lists without allocation; must accommodate a full byte-sized list.
#[cfg(not(feature = "alloc"))]
#[env_item("ZIGBEE_ZDP_CLUSTER_CAPACITY")]
pub const CLUSTER_CAPACITY: usize = 255;
#[cfg(not(feature = "alloc"))]
const _: () = {
    assert!(TLV_CAPACITY > 0, "ZDP TLV capacity must be positive");
    assert!(
        ADDRESS_CAPACITY >= 255,
        "ZDP address capacity must be at least 255"
    );
    assert!(
        CLUSTER_CAPACITY >= 255,
        "ZDP cluster capacity must be at least 255"
    );
};

#[cfg(not(feature = "alloc"))]
pub(crate) type Prefix = heapless::Vec<u8, PREFIX_CAPACITY>;

/// Boxed ownership with `alloc`, inline ownership otherwise.
#[cfg(feature = "alloc")]
pub type Owned<T> = Box<T>;
/// Boxed ownership with `alloc`, inline ownership otherwise.
#[cfg(not(feature = "alloc"))]
pub type Owned<T> = T;
/// Unprefixed TLV vector.
#[cfg(feature = "alloc")]
pub type Tlvs = Vec<Tlv>;
/// Unprefixed bounded TLV vector.
#[cfg(not(feature = "alloc"))]
pub type Tlvs = RawList<Tlv, TLV_CAPACITY>;
/// Unprefixed owned TLV slice.
#[cfg(feature = "alloc")]
pub type TlvSlice = Box<[Tlv]>;
/// Unprefixed bounded TLV slice replacement.
#[cfg(not(feature = "alloc"))]
pub type TlvSlice = Tlvs;
/// Owned associated network addresses.
#[cfg(feature = "alloc")]
pub type Addresses = Box<[u16]>;
/// Bounded associated network addresses.
#[cfg(not(feature = "alloc"))]
pub type Addresses = heapless::Vec<u16, ADDRESS_CAPACITY>;
/// Owned cluster IDs returned by a descriptor.
#[cfg(feature = "alloc")]
pub type ClusterIds = Box<[u16]>;
/// Bounded cluster IDs returned by a descriptor.
#[cfg(not(feature = "alloc"))]
pub type ClusterIds = heapless::Vec<u16, CLUSTER_CAPACITY>;

#[cfg_attr(
    not(feature = "alloc"),
    expect(
        clippy::missing_const_for_fn,
        reason = "allocating mode must deallocate its box"
    )
)]
#[cfg_attr(
    feature = "alloc",
    expect(
        clippy::boxed_local,
        reason = "consume the existing boxed public API without cloning"
    )
)]
pub(crate) fn unpack<T>(value: Owned<T>) -> T {
    #[cfg(feature = "alloc")]
    {
        *value
    }
    #[cfg(not(feature = "alloc"))]
    {
        value
    }
}

pub(crate) fn single_tlv(tlv: Tlv) -> Tlvs {
    #[cfg(feature = "alloc")]
    {
        vec![tlv]
    }
    #[cfg(not(feature = "alloc"))]
    {
        let mut result = Tlvs::default();
        result.push(tlv).expect("TLV capacity is positive");
        result
    }
}

/// A bounded ZDP collection cannot hold another element.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("ZDP collection capacity exceeded: {capacity} elements")]
pub struct CapacityError {
    /// Maximum element count.
    pub capacity: usize,
}

#[cfg(test)]
#[cfg_attr(
    not(feature = "alloc"),
    expect(
        clippy::missing_const_for_fn,
        reason = "allocating mode allocates its box"
    )
)]
pub(crate) fn own<T>(value: T) -> Owned<T> {
    #[cfg(feature = "alloc")]
    {
        Box::new(value)
    }
    #[cfg(not(feature = "alloc"))]
    {
        value
    }
}
