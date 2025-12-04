//! Cluster groups.

pub mod general;
pub mod lighting;

/// Commands for all clusters.
#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Command {
    /// General cluster commands.
    General(general::Command),
    /// Lighting cluster commands.
    Lighting(lighting::Command),
}

impl From<general::Command> for Command {
    fn from(cmd: general::Command) -> Self {
        Self::General(cmd)
    }
}

impl From<lighting::Command> for Command {
    fn from(cmd: lighting::Command) -> Self {
        Self::Lighting(cmd)
    }
}

impl From<general::on_off::On> for Command {
    fn from(cmd: general::on_off::On) -> Self {
        Self::General(general::Command::OnOff(general::on_off::Command::On(cmd)))
    }
}

impl From<general::on_off::Off> for Command {
    fn from(cmd: general::on_off::Off) -> Self {
        Self::General(general::Command::OnOff(general::on_off::Command::Off(cmd)))
    }
}

impl From<lighting::color_control::MoveToColor> for Command {
    fn from(cmd: lighting::color_control::MoveToColor) -> Self {
        Self::Lighting(lighting::Command::ColorControl(
            lighting::color_control::Command::MoveToColor(cmd),
        ))
    }
}

/// Responses for all clusters.
#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Response {
    /// General cluster responses.
    General(general::Response),
    /// Lighting cluster responses.
    Lighting(lighting::Response),
}
