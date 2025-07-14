//! Lighting API.

pub use commands::*;
pub use direction::Direction;

use crate::zcl::Cluster;

mod color_information_attribute;
mod commands;
mod direction;
pub mod mode;

trait Lighting {}

impl<T> Cluster for T
where
    T: Lighting,
{
    const ID: u16 = 0x0300;
}
