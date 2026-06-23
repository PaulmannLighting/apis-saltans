use le_stream::{LeStream, LeStreamIterator, ToLeStream};

use crate::types::tlv::{EncapsulatedGlobal, General, Local, Payload, Tag, Tlv};

/// Beacon Appendix Encapsulation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BeaconAppendixEncapsulation {
    bytes: Payload,
}

impl BeaconAppendixEncapsulation {
    /// Creates a new `BeaconAppendixEncapsulation`.
    #[must_use]
    pub fn new<T>(tlvs: T) -> Option<Self>
    where
        T: IntoIterator<Item = Tlv<Local, EncapsulatedGlobal>>,
    {
        let bytes: Payload = tlvs
            .into_iter()
            .flat_map(ToLeStream::to_le_stream)
            .collect();

        if bytes.is_empty() {
            return None;
        }

        Some(Self { bytes })
    }
}

impl Tag for BeaconAppendixEncapsulation {
    const TAG: u8 = 73;
}

impl From<Payload> for BeaconAppendixEncapsulation {
    fn from(bytes: Payload) -> Self {
        Self { bytes }
    }
}

impl From<BeaconAppendixEncapsulation> for General {
    fn from(value: BeaconAppendixEncapsulation) -> Self {
        Self::new(BeaconAppendixEncapsulation::TAG, value.bytes)
    }
}

impl IntoIterator for BeaconAppendixEncapsulation {
    type Item = Tlv<Local, EncapsulatedGlobal>;

    type IntoIter = LeStreamIterator<Self::Item, <Payload as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter().le_stream()
    }
}
