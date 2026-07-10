use std::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};
use zb_core::types::Type;

use crate::Status;

/// Read Attributes Status Record.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ReadAttributesStatus {
    attribute_id: u16,
    data: Result<Type, Result<Status, u8>>,
}

impl ReadAttributesStatus {
    /// Returns the attribute ID.
    pub fn into_parts(self) -> (u16, Result<Type, Result<Status, u8>>) {
        (self.attribute_id, self.data)
    }
}

impl FromLeStream for ReadAttributesStatus {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let attribute_id = u16::from_le_stream(&mut bytes)?;
        let status = Status::try_from(u8::from_le_stream(&mut bytes)?);

        let data = if Ok(Status::Success) == status {
            Ok(Type::from_le_stream(bytes)?)
        } else {
            Err(status)
        };

        Some(Self { attribute_id, data })
    }
}

impl ToLeStream for ReadAttributesStatus {
    type Iter = Chain<
        Chain<<u16 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>,
        <Option<Type> as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        let (status, data) = match self.data {
            Ok(typ) => (Status::Success.into(), Some(typ)),
            Err(error) => match error {
                Ok(status) => (status.into(), None),
                Err(status) => (status, None),
            },
        };

        self.attribute_id
            .to_le_stream()
            .chain(status.to_le_stream())
            .chain(data.to_le_stream())
    }
}
