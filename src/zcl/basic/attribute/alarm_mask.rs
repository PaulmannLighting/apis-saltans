use bitflags::bitflags;
use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;

/// Alarm mask.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    FromPrimitive,
    FromLeStream,
    ToLeStream,
)]
#[repr(transparent)]
pub struct AlarmMask(u8);

bitflags! {
    impl AlarmMask: u8 {
        /// General hardware fault.
        const GENERAL_HARDWARE_FAULT = 0b0000_0001;
        /// General software fault.
        const GENERAL_SOFTWARE_FAULT = 0b0000_0010;
    }
}

impl AlarmMask {
    /// Create a new `AlarmMask`.
    #[must_use]
    pub const fn new(mask: u8) -> Self {
        Self(mask)
    }

    /// Return whether this is a general hardware fault.
    #[must_use]
    pub const fn is_general_hardware_fault(self) -> bool {
        self.contains(Self::GENERAL_HARDWARE_FAULT)
    }

    /// Return whether this is a general software fault.
    #[must_use]
    pub const fn is_general_software_fault(self) -> bool {
        self.contains(Self::GENERAL_SOFTWARE_FAULT)
    }
}
