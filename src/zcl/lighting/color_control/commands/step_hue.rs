//! Data structures for the `Step Hue` command in the `Lighting` cluster.

pub use mode::Mode;

pub(in crate::zcl::lighting::color_control) mod command;
mod mode;
