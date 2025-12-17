use le_stream::FromLeStream;
use zigbee::types::Type;

use crate::Status;

/// Read Attributes Status Record.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ReadAttributesStatus {
    attribute_id: u16,
    data: Result<Type, u8>,
}

impl ReadAttributesStatus {
    /// Returns the attribute ID.
    pub fn into_parts(self) -> (u16, Result<Type, u8>) {
        (self.attribute_id, self.data)
    }
}

impl FromLeStream for ReadAttributesStatus {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let attribute_id = u16::from_le_stream(&mut bytes)?;
        let status = u8::from_le_stream(&mut bytes)?;

        let data = if Status::try_from(status) == Ok(Status::Success) {
            Ok(Type::from_le_stream(bytes)?)
        } else {
            Err(status)
        };

        Some(Self { attribute_id, data })
    }
}
