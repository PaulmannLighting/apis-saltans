#![cfg(feature = "driver")]

//! Re-exported protocol crates for driver implementors.
//!
//! These modules expose the APIS Saltans protocol types needed by hardware
//! drivers without requiring backend crates to depend on the protocol crates
//! directly.

/// APS frame and payload types.
pub mod aps {
    pub use zb_aps::*;
}

/// Core Zigbee domain types.
pub mod core {
    pub use zb_core::*;
}

/// NWK receive context types.
pub mod nwk {
    pub use zb_nwk::*;
}

/// ZDP descriptor and service types.
pub mod zdp {
    pub use zb_zdp::*;
}
