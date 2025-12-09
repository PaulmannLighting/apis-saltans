use crate::general::on_off::{Off, OffWithEffect, On, Toggle};

/// Available ZCL frames.
// TODO: Add all ZCL commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Frames {
    /// On command.
    On(On),
    /// Off command.
    Off(Off),
    /// Off with Effect command.
    OffWithEffect(OffWithEffect),
    /// Toggle command.
    Toggle(Toggle),
}
