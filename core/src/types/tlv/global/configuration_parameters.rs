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
        const AIB = 0b0000_0000_0000_0001;
        /// If this bit is set, Security Policy configuration is supported.
        const DEVICE_SECURITY_POLICY = 0b0000_0000_0000_0010;
        /// If this bit is set, Network Information Base configuration is supported.
        const NIB = 0b0000_0000_0000_0100;
    }
}

impl_bitflags_display_and_from_str!(ConfigurationParameters);

impl Tag for ConfigurationParameters {
    const TAG: u8 = 75;
}
