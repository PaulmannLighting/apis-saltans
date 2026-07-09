//! Attributes of the Identify cluster.

use apis_saltans_core::Cluster;
use apis_saltans_core::types::Uint16;

use crate::macros::zcl_attributes;

zcl_attributes! {
    cluster: Cluster::Identify;

    /// Remaining length of time, in seconds, that the device will continue to identify itself.
    IdentifyTime = 0x0000: Uint16 { R, W },
}
