//! Application Support Sublayer Data Entity service primitives.
//!
//! The APSDE service-access point transports application-service data units
//! between next-higher-layer entities and the APS sublayer. This crate models
//! the `APSDE-DATA.request`, `APSDE-DATA.confirm`, and
//! `APSDE-DATA.indication` primitives without coupling them to an actor,
//! hardware backend, or wire-frame representation.
//!
//! Addressing enums encode the fields permitted by each primitive. Generic
//! ASDU, timestamp, and device-key-pair types let an implementation retain its
//! native representations, while propagated status codes remain lossless.

pub use self::address::{
    AddressMode, BroadcastAddress, Destination, IndividualEndpoint, NetworkAddress,
    ReceivedDestination, RequestDestination, Source,
};
pub use self::alias::Alias;
pub use self::confirm::{ConfirmStatus, DataConfirm};
pub use self::indication::{DataIndication, IndicationMetadata, IndicationStatus};
pub use self::request::DataRequest;
pub use self::security::{Security, SecurityStatus};
pub use self::status::Status;
pub use self::tx_options::TxOptions;

mod address;
mod alias;
mod confirm;
mod indication;
mod request;
mod security;
mod status;
mod tx_options;
