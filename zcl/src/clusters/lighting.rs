//! Lighting API.

pub mod ballast_configuration;
pub mod color_control;

/// Commands for the Lighting clusters.
#[derive(Debug)]
pub enum Command {
    /// Ballast Configuration cluster commands.
    BallastConfiguration(ballast_configuration::Command),
    /// Color Control cluster commands.
    ColorControl(color_control::Command),
}

/// Responses for the Lighting clusters.
#[derive(Debug)]
pub enum Response {
    /// Ballast Configuration cluster responses.
    BallastConfiguration(ballast_configuration::Response),
    /// Color Control cluster responses.
    ColorControl(color_control::Response),
}
