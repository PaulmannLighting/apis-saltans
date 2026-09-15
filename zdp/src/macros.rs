//! Crate-local protocol macros, grouped by responsibility.

pub(crate) use self::command::zdp_command;
pub(crate) use self::command_enum::zdp_command_enum;
pub(crate) use self::command_group::zdp_command_group;

mod command;
mod command_enum;
mod command_group;
