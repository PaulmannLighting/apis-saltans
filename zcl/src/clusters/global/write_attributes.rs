//! Write Attributes Command and Response.

use core::iter::{Once, once};
use core::ops::Deref;

use either::Either;
use le_stream::ToLeStream;
use zb_core::Direction;

pub use self::record::Record;
pub use self::status::Status;
use crate::macros::zcl_command;
use crate::storage::Records;

mod record;
#[cfg(feature = "serde")]
mod response_records;
mod status;
#[cfg(test)]
mod tests;

const SUCCESS: u8 = crate::Status::Success as u8;

zcl_command! {
    /// Write Attributes Command.
    Command {
        Global;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        response: Response;
        => crate::global::WriteAttributes;
        fields {
            records: Records<Record>,
        }
    }
}

impl Deref for Command {
    type Target = [Record];

    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

zcl_command! {
    /// Write Attributes response with either request-wide success or per-attribute failures.
    Response {
        Global;
        command_id: 0x04;
        direction: Direction::ServerToClient;
        => crate::global::WriteAttributesResponse;
        fields {
            #[cfg_attr(feature = "serde", serde(with = "response_records"))]
            records: Option<Records<Status>>,
        }

        constructor {
            /// Create a response from status records. Successful records are omitted.
            /// An empty list or a list with only successful records indicates request-wide success.
            #[must_use]
            pub fn new(records: Records<Status>) -> Self {
                #[cfg(feature = "alloc")]
                let mut records = records.into_vec();
                #[cfg(not(feature = "alloc"))]
                let mut records = records;
                records.retain(|record| !record.is_success());
                #[cfg(feature = "alloc")]
                let records = records.into_boxed_slice();
                Self { records: (!records.is_empty()).then_some(records) }
            }

            /// Create a request-wide success response.
            #[must_use]
            pub const fn success() -> Self {
                Self { records: None }
            }
        }

        getters {
            /// Return whether every requested write succeeded.
            #[must_use]
            pub const fn is_success(&self) -> bool {
                self.records.is_none()
            }

            /// Return the failed writes in wire order.
            #[must_use]
            pub fn failures(&self) -> &[Status] {
                self.records.as_deref().unwrap_or_default()
            }
        }

        from_le_stream {
            fn from_le_stream<I>(bytes: I) -> Option<Self>
            where
                I: Iterator<Item = u8>,
            {
                let mut bytes = bytes.peekable();
                let first = bytes.next()?;
                if first == SUCCESS && bytes.peek().is_none() {
                    return Some(Self::success());
                }
                let records = Records::<Status>::from_le_stream(once(first).chain(bytes))?;
                Some(Self::new(records))
            }
        }

        to_le_stream {
            type Iter = Either<Once<u8>, <Records<Status> as ToLeStream>::Iter>;

            fn to_le_stream(self) -> Self::Iter {
                self.records.map_or_else(
                    || Either::Left(once(SUCCESS)),
                    |records| Either::Right(records.to_le_stream()),
                )
            }
        }
    }
}

impl Deref for Response {
    type Target = [Status];

    fn deref(&self) -> &Self::Target {
        self.failures()
    }
}

impl IntoIterator for Response {
    type Item = <Records<Status> as IntoIterator>::Item;
    type IntoIter = <Records<Status> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.records.unwrap_or_default().into_iter()
    }
}
