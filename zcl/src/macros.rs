//! Crate-local protocol macros, grouped by responsibility.

pub(crate) use self::attribute_newtype::zcl_attribute_newtype;
pub(crate) use self::attributes::zcl_attributes;
pub(crate) use self::bitflags::impl_bitflags_display_and_from_str;
pub(crate) use self::command::zcl_command;
pub(crate) use self::command_enum::zcl_command_enum;
pub(crate) use self::profile::zcl_cluster_profile;

mod attribute_newtype;
mod attributes;
mod bitflags;
mod command;
mod command_enum;
mod profile;
#[cfg(test)]
mod tests;
