use std::iter::Chain;

use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Router Information TLV structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RouterInformation(u16);

bitflags! {
    impl RouterInformation: u16 {
        /// If this bit is set, there is hub connectivity.
        const HUB_CONNECTIVITY = 0b1000_0000_0000_0000;
        /// If this bit is set, the router is up for more than 24 hours.
        const UPTIME = 0b0100_0000_0000_0000;
        /// If this bit is set, this device is a preferred parent.
        const PREFERRED_PARENT = 0b0010_0000_0000_0000;
        /// If this bit is set, the device has battery backup.
        const BATTERY_BACKUP = 0b0001_0000_0000_0000;
        /// If this bit is set, the device supports enhanced beacon requests.
        const ENHANCED_BEACON_REQUEST_SUPPORT = 0b0000_1000_0000_0000;
        /// If this bit is set, the device supports MAC data poll keepalive.
        const MAC_DATA_POLL_KEEPALIVE_SUPPORT = 0b0000_0100_0000_0000;
        /// If this bit is set, the device supports end device keepalive.
        const END_DEVICE_KEEPALIVE_SUPPORT = 0b0000_0010_0000_0000;
        /// If this bit is set, the device supports power negotiation.
        const POWER_NEGOTIATION_SUPPORT = 0b0000_0001_0000_0000;
    }
}

impl FromLeStream for RouterInformation {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        u16::from_le_stream(bytes).map(Self::from_bits_retain)
    }
}

impl Tag for RouterInformation {
    const TAG: u8 = 70;

    fn size(&self) -> usize {
        2
    }
}

impl ToLeStream for RouterInformation {
    type Iter =
        Chain<Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>, <u16 as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.0.to_le_stream())
    }
}
