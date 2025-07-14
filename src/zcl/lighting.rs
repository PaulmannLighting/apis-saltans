//! Lighting API.

pub use commands::*;
pub use misc::{color_loop_set, move_hue, move_saturation, move_to_hue, step_hue, step_saturation};

use crate::zcl::Cluster;

mod color_information_attribute;
mod commands;
mod misc;

const CLUSTER_ID: u16 = 0x0300;

/// Sealed trait for the Lighting cluster.
trait Lighting {}

impl<T> Cluster for T
where
    T: Lighting,
{
    const ID: u16 = CLUSTER_ID;
}
