//! Common types used across the protocol.

pub use analog::{
    Int8, Int16, Int24, Int32, Int40, Int48, Int56, Int64, Uint8, Uint16, Uint24, Uint32, Uint40,
    Uint48, Uint56, Uint64,
};
pub use composite::{OctStr, OctStr16, String, String16};
pub use discrete::{Bool, Data8, Data16, Data24, Data32, Data40, Data48, Data56, Data64};
pub use null::{NoData, Unknown};

mod analog;
mod composite;
mod discrete;
mod null;
