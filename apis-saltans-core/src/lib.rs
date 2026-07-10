//! Shared Zigbee protocol value types.
//!
//! This crate contains the small domain types used across the APIS Saltans
//! workspace: IEEE addresses, NWK short addresses, APS endpoints and
//! destinations, profile and cluster identifiers, node descriptors, ZCL value
//! types, TLVs, and protocol unit wrappers.
//!
//! Address-like values encode Zigbee invariants where practical. For example,
//! [`Endpoint`] separates the ZDO data endpoint, application endpoints, the
//! broadcast endpoint, and the reserved endpoint range. APIs that deserialize
//! raw protocol bytes can keep those bytes losslessly and expose fallible
//! getters when a reserved value must be reported to callers.

#![no_std]

pub use self::byte_sized_vec::ByteSizedVec;
pub use self::cluster::{Cluster, ClusterSpecific};
pub use self::destination::Destination;
pub use self::direction::Direction;
pub use self::endpoint::{Application, Endpoint};
pub use self::full_address::FullAddress;
pub use self::group_id::GroupId;
pub use self::ieee_address::{Eui64, IeeeAddress};
pub use self::profile::{Profile, Profiled};
pub use self::short_id::ShortId;
pub use self::traits::ExpectResponse;

mod byte_sized_vec;
mod cluster;
pub mod constants;
mod full_address;
#[macro_use]
mod macros;
/// Outbound Zigbee destination types.
pub mod destination;
mod direction;
/// Zigbee endpoint domain types.
pub mod endpoint;
mod group_id;
mod ieee_address;
pub mod node;
mod profile;
/// Zigbee NWK short-address domain types.
pub mod short_id;
mod traits;
pub mod types;
pub mod units;
