#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum StartUpOnOff {
    /// Set the `OnOff` attribute to 0 (off).
    Off = 0x00,
    /// Set the `OnOff` attribute to 1 (on).
    On = 0x01,
    /// If the previous value of the `OnOff` attribute is equal to 0, set the `OnOff`
    /// attribute to 1. If the previous value of the `OnOff` attribute is equal to 1,
    /// set the `OnOff` attribute to 0 (toggle).
    Toggle = 0x02,
    /// Set the `OnOff` attribute to its previous value.
    Previous = 0xff,
}
