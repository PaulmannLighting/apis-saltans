use le_stream::{LeStream, LeStreamIterator, ToLeStream};

use crate::types::tlv::{EncapsulatedGlobal, General, Local, Payload, Tag, Tlv};

/// Joiner Encapsulation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct JoinerEncapsulation {
    bytes: Payload,
}

impl JoinerEncapsulation {
    /// Creates a new `JoinerEncapsulation`.
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

impl Tag for JoinerEncapsulation {
    const TAG: u8 = 72;
}

impl From<Payload> for JoinerEncapsulation {
    fn from(bytes: Payload) -> Self {
        Self { bytes }
    }
}

impl From<JoinerEncapsulation> for General {
    fn from(value: JoinerEncapsulation) -> Self {
        Self::new(JoinerEncapsulation::TAG, value.bytes)
    }
}

impl IntoIterator for JoinerEncapsulation {
    type Item = Tlv<Local, EncapsulatedGlobal>;

    type IntoIter = LeStreamIterator<Self::Item, <Payload as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter().le_stream()
    }
}
