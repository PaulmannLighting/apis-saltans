//! Commands for the Basic cluster.

use crate::zcl::basic::CLUSTER_ID;
use crate::{Cluster, Command};

/// Reset a device to factory defaults.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResetToFactoryDefaults;

impl Cluster for ResetToFactoryDefaults {
    const ID: u16 = CLUSTER_ID;
}

impl Command for ResetToFactoryDefaults {
    const ID: u8 = 0x00;
}
