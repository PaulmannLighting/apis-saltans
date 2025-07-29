//! Common types used across the protocol.

pub use analog::{Uint8, Uint16, Uint24, Uint32, Uint40, Uint48};
pub use discrete::{Bool, Data8, Data16, Data24, Data32, Data40, Data48, Data56, Data64};
pub use null::{NoData, Unknown};
pub use oct_str::OctStr;
pub use oct_str16::OctStr16;
pub use string::String;
pub use string16::String16;

mod analog;
mod discrete;
mod null;
mod oct_str;
mod oct_str16;
mod string;
mod string16;
