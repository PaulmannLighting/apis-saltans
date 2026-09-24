use core::iter;

use le_stream::{FromLeStream, ToLeStream};

/// Status of an attribute reporting configuration.
///
/// Success applies to the entire request and omits direction and attribute ID.
/// Failure records contain both fields.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "StatusFields"))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AttributeStatus {
    status: u8,
    direction: Option<u8>,
    attribute_id: Option<u16>,
}

impl AttributeStatus {
    /// Creates a status record, omitting direction and attribute ID on success.
    #[must_use]
    pub const fn new(status: u8, direction: u8, attribute_id: u16) -> Self {
        if status == crate::Status::Success as u8 {
            Self::success()
        } else {
            Self {
                status,
                direction: Some(direction),
                attribute_id: Some(attribute_id),
            }
        }
    }

    /// Creates a success record for all requested attributes.
    #[must_use]
    pub const fn success() -> Self {
        Self {
            status: crate::Status::Success as u8,
            direction: None,
            attribute_id: None,
        }
    }

    /// Returns the status.
    #[must_use]
    pub const fn status(&self) -> u8 {
        self.status
    }

    /// Returns the direction for a failure, or `None` on success.
    #[must_use]
    pub const fn direction(&self) -> Option<u8> {
        self.direction
    }

    /// Returns the attribute ID for a failure, or `None` on success.
    #[must_use]
    pub const fn attribute_id(&self) -> Option<u16> {
        self.attribute_id
    }
}

impl FromLeStream for AttributeStatus {
    fn from_le_stream<T>(mut stream: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let status = stream.next()?;
        if status == crate::Status::Success as u8 {
            Some(Self::success())
        } else {
            Some(Self::new(
                status,
                stream.next()?,
                u16::from_le_stream(stream)?,
            ))
        }
    }
}

impl ToLeStream for AttributeStatus {
    type Iter = iter::Chain<
        iter::Chain<iter::Once<u8>, std::option::IntoIter<u8>>,
        iter::Flatten<std::option::IntoIter<[u8; size_of::<u16>()]>>,
    >;

    fn to_le_stream(self) -> Self::Iter {
        iter::once(self.status).chain(self.direction).chain(
            self.attribute_id
                .map(u16::to_le_bytes)
                .into_iter()
                .flatten(),
        )
    }
}

/// Intermediate serde representation validated before constructing a record.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct StatusFields {
    status: u8,
    direction: Option<u8>,
    attribute_id: Option<u16>,
}

#[cfg(feature = "serde")]
impl TryFrom<StatusFields> for AttributeStatus {
    type Error = &'static str;

    fn try_from(fields: StatusFields) -> Result<Self, Self::Error> {
        match (fields.status, fields.direction, fields.attribute_id) {
            (status, None, None) if status == crate::Status::Success as u8 => Ok(Self::success()),
            (status, Some(direction), Some(attribute_id))
                if status != crate::Status::Success as u8 =>
            {
                Ok(Self::new(status, direction, attribute_id))
            }
            _ => Err(
                "direction and attribute ID must both be absent on success and present on failure",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use le_stream::{FromLeStream, ToLeStream};

    use super::AttributeStatus;

    #[test]
    fn success_omits_fields_and_consumes_only_status() {
        let mut bytes = [0x00, 0x86].into_iter();
        let record = AttributeStatus::from_le_stream(&mut bytes).unwrap();
        assert_eq!(record, AttributeStatus::success());
        assert_eq!(record.direction(), None);
        assert_eq!(record.attribute_id(), None);
        assert_eq!(bytes.next(), Some(0x86));
        assert_eq!(record.to_le_stream().collect::<Vec<_>>(), [0x00]);
        assert_eq!(AttributeStatus::new(0x00, 0x01, 0x1234), record);
    }

    #[test]
    fn failure_requires_both_fields() {
        const PAYLOAD: [u8; 4] = [0x86, 0x01, 0x34, 0x12];
        let record = AttributeStatus::from_le_stream(PAYLOAD.into_iter()).unwrap();
        assert_eq!(record.direction(), Some(0x01));
        assert_eq!(record.attribute_id(), Some(0x1234));
        assert_eq!(record.to_le_stream().collect::<Vec<_>>(), PAYLOAD);
        for length in 0..PAYLOAD.len() {
            assert_eq!(
                AttributeStatus::from_le_stream(PAYLOAD[..length].iter().copied()),
                None
            );
        }
    }
}
