use le_stream::{FromLeStream, ToLeStream};

/// Write Attributes Status record.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct Status {
    status: u8,
    id: u16,
}

impl Status {
    /// Create a new status record.
    #[must_use]
    pub const fn new(status: crate::Status, id: u16) -> Self {
        Self {
            status: status as u8,
            id,
        }
    }

    /// Return the status code.
    ///
    /// # Errors
    ///
    /// Returns the raw status code if it is not a valid status.
    pub fn status(self) -> Result<crate::Status, u8> {
        self.status.try_into()
    }

    /// Return the attribute ID.
    #[must_use]
    pub const fn id(self) -> u16 {
        self.id
    }
}

impl TryFrom<Status> for u16 {
    type Error = Self;

    /// Try to convert a status record into a successfully read attribute ID.
    ///
    /// # Errors
    ///
    /// Returns the attribute ID in the `Err` variant if the status code is not `Success`.
    fn try_from(value: Status) -> Result<Self, Self::Error> {
        if let Ok(status) = value.status()
            && status == crate::Status::Success
        {
            Ok(value.id)
        } else {
            Err(value.id)
        }
    }
}
