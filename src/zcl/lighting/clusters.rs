use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum Clusters {
    /// Attributes and commands for controlling the color of a color-capable light.
    ColorControl = 0x0300,
    /// Attributes and commands for configuring a lighting ballast.
    BallastConfiguration = 0x0301,
}
