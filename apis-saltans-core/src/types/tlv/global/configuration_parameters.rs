use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Configuration Parameters bitmask.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
#[repr(transparent)]
pub struct ConfigurationParameters(u16);

bitflags! {
    impl ConfigurationParameters: u16 {
        /// If this bit is set, AIB configuration is supported.
        const AIB = 0b1000_0000_0000_0000;
        /// If this bit is set, Security Policy configuration is supported.
        const DEVICE_SECURITY_POLICY = 0b0100_0000_0000_0000;
        /// If this bit is set, Network Information Base configuration is supported.
        const NIB = 0b0010_0000_0000_0000;
    }
}

impl Tag for ConfigurationParameters {
    const TAG: u8 = 75;
}
