use std::iter::Chain;

use bitflags::bitflags;
use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::Tag;

/// Configuration Parameters bitmask.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

    fn size(&self) -> usize {
        2
    }
}

impl FromLeStreamTagged for ConfigurationParameters {
    type Tag = u8;

    fn from_le_stream_tagged<T>(length: Self::Tag, mut bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        let Some(size) = usize::from(length).checked_add(1) else {
            return Err(length);
        };

        if size != 2 {
            return Err(length);
        }

        Ok(u16::from_le_stream(&mut bytes).map(Self::from_bits_truncate))
    }
}

impl ToLeStream for ConfigurationParameters {
    type Iter =
        Chain<Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>, <u16 as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.bits().to_le_stream())
    }
}
