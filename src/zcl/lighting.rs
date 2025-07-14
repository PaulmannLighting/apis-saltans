//! Lighting API.

pub use commands::*;
pub use direction::Direction;
pub use mode::{move_hue, move_saturation, step_hue, step_saturation};

use crate::zcl::Cluster;

mod color_information_attribute;
mod commands;
mod direction;
mod mode;

trait Lighting {}

impl<T> Cluster for T
where
    T: Lighting,
{
    const ID: u16 = 0x0300;
}
