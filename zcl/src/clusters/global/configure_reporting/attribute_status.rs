use core::iter;

use le_stream::{FromLeStream, ToLeStream};

use crate::Status;

/// Status of an attribute reporting configuration.
///
/// Deserialization reads the status and optional direction and attribute ID without
/// status-dependent validation. Byte serialization omits both optional fields on success.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream)]
pub struct AttributeStatus {
    status: u8,
    direction: Option<u8>,
    attribute_id: Option<u16>,
}

impl AttributeStatus {
    /// Creates a status record, preserving the supplied optional fields.
    #[must_use]
    pub const fn new(status: Status, direction: Option<u8>, attribute_id: Option<u16>) -> Self {
        Self {
            status: status as u8,
            direction,
            attribute_id,
        }
    }

    /// Creates a success record for all requested attributes.
    #[must_use]
    pub const fn success() -> Self {
        Self {
            status: Status::Success as u8,
            direction: None,
            attribute_id: None,
        }
    }

    /// Returns the status.
    #[must_use]
    pub const fn status(&self) -> u8 {
        self.status
    }

    /// Returns the direction, if present.
    #[must_use]
    pub const fn direction(&self) -> Option<u8> {
        self.direction
    }

    /// Returns the attribute ID, if present.
    #[must_use]
    pub const fn attribute_id(&self) -> Option<u16> {
        self.attribute_id
    }
}

impl ToLeStream for AttributeStatus {
    type Iter = iter::Chain<
        iter::Chain<iter::Once<u8>, std::option::IntoIter<u8>>,
        iter::Flatten<std::option::IntoIter<[u8; size_of::<u16>()]>>,
    >;

    fn to_le_stream(self) -> Self::Iter {
        let (direction, attribute_id) = if self.status == Status::Success as u8 {
            (None, None)
        } else {
            (self.direction, self.attribute_id)
        };
        iter::once(self.status)
            .chain(direction)
            .chain(attribute_id.map(u16::to_le_bytes).into_iter().flatten())
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};

    use super::AttributeStatus;

    #[test]
    fn success_reads_fields_but_omits_them_on_serialization() {
        let mut bytes = [0x00, 0x01, 0x34, 0x12, 0x86].into_iter();
        let record = AttributeStatus::from_le_stream(&mut bytes).unwrap();
        assert_eq!(record.status(), 0x00);
        assert_eq!(record.direction(), Some(0x01));
        assert_eq!(record.attribute_id(), Some(0x1234));
        assert_eq!(bytes.next(), Some(0x86));
        assert_eq!(record.to_le_stream().collect::<Vec<_>>(), [0x00]);
    }

    #[test]
    fn parses_status_without_optional_fields() {
        assert_eq!(
            AttributeStatus::from_le_stream([0x00].into_iter()),
            Some(AttributeStatus::success()),
        );
        assert_eq!(AttributeStatus::from_le_stream([].into_iter()), None);
    }

    #[test]
    fn rejects_partial_attribute_id() {
        assert_eq!(
            AttributeStatus::from_le_stream([0x86, 0x01, 0x34].into_iter()),
            None
        );
        assert_eq!(
            AttributeStatus::from_le_stream([0x00, 0x01, 0x34].into_iter()),
            None
        );
    }

    #[test]
    fn failure_preserves_available_fields() {
        const PAYLOAD: [u8; 4] = [0x86, 0x01, 0x34, 0x12];
        const STATUS_LENGTH: usize = 1;
        const STATUS_AND_DIRECTION_LENGTH: usize = 2;
        for length in [STATUS_LENGTH, STATUS_AND_DIRECTION_LENGTH, PAYLOAD.len()] {
            let record =
                AttributeStatus::from_le_stream(PAYLOAD[..length].iter().copied()).unwrap();
            assert_eq!(record.status(), 0x86);
            assert_eq!(record.direction(), (length > STATUS_LENGTH).then_some(0x01));
            assert_eq!(
                record.attribute_id(),
                (length == PAYLOAD.len()).then_some(0x1234)
            );
            assert_eq!(record.to_le_stream().collect::<Vec<_>>(), PAYLOAD[..length]);
        }
    }
}
