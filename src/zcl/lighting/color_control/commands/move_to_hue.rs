//! Data structures for the `Move To Hue` command in the `Lighting` cluster.

pub use direction::Direction;

pub(in crate::zcl::lighting::color_control) mod command;
mod direction;
