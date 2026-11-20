//! Shared wire format for management table response pages.

#[cfg(feature = "alloc")]
use alloc::vec::{IntoIter, Vec};

use zb_core::ByteSizedVec;

use crate::Status;
#[cfg(not(feature = "alloc"))]
use crate::storage::Stream;

const BINDING_HEADER_BYTES: usize = 12;
const GROUP_MODE: u8 = 0x01;
const EXTENDED_MODE: u8 = 0x03;
const GROUP_ADDRESS_BYTES: usize = 2;
const EXTENDED_ADDRESS_AND_ENDPOINT_BYTES: usize = 9;
const NEIGHBOR_ENTRY_BYTES: usize = 22;
const ROUTING_ENTRY_BYTES: usize = 5;
const STATUS_BYTES: usize = 1;

#[cfg(feature = "alloc")]
type TableBytes = Vec<u8>;
#[cfg(not(feature = "alloc"))]
type TableBytes = ByteSizedVec<u8>;
#[cfg(feature = "alloc")]
pub(super) type TableIter = IntoIter<u8>;
#[cfg(not(feature = "alloc"))]
pub(super) type TableIter = Stream<<TableBytes as IntoIterator>::IntoIter>;

/// Wire layout of one entry in a management response.
#[derive(Clone, Copy)]
pub(super) enum EntryLayout {
    Binding,

    Neighbor,

    Routing,
}

/// A management response header and the bytes of its validated entries.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(super) struct Table {
    pub(super) status: u8,
    pub(super) total_entries: u8,
    pub(super) start_index: u8,
    pub(super) list_count: u8,
    pub(super) list_bytes: TableBytes,
}

impl EntryLayout {
    /// Consume one complete entry, rejecting truncation and unsupported binding modes.
    fn read<I>(self, bytes: &mut I, list: &mut TableBytes) -> Option<()>
    where
        I: Iterator<Item = u8>,
    {
        let length = match self {
            Self::Binding => {
                read_bytes(bytes, list, BINDING_HEADER_BYTES)?;
                match *list.last()? {
                    GROUP_MODE => GROUP_ADDRESS_BYTES,
                    EXTENDED_MODE => EXTENDED_ADDRESS_AND_ENDPOINT_BYTES,
                    _ => return None,
                }
            }
            Self::Neighbor => NEIGHBOR_ENTRY_BYTES,
            Self::Routing => ROUTING_ENTRY_BYTES,
        };

        read_bytes(bytes, list, length)
    }
}

impl Table {
    /// Build a success page from the legacy raw-byte payload or return an invalid-request status.
    pub(super) fn from_list(
        total_entries: u8,
        start_index: u8,
        bytes: ByteSizedVec<u8>,
        layout: EntryLayout,
    ) -> Self {
        let Some((list_bytes, list_count)) = read_entries(bytes.into_iter(), layout, u8::MAX)
        else {
            return Self::failed(Status::InvalidRequestType.into());
        };

        Self {
            status: Status::Success.into(),
            total_entries,
            start_index,
            list_count,
            list_bytes,
        }
    }

    /// Build a status-only response.
    pub(super) const fn failed(status: u8) -> Self {
        Self {
            status,
            total_entries: u8::MIN,
            start_index: u8::MIN,
            list_count: u8::MIN,
            list_bytes: TableBytes::new(),
        }
    }

    /// Decode the response status and validate the declared entry count on success.
    pub(super) fn decode<I>(mut bytes: I, layout: EntryLayout) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let status = bytes.next()?;
        if status != Status::Success as u8 {
            return Some(Self::failed(status));
        }

        let total_entries = bytes.next()?;
        let start_index = bytes.next()?;
        let list_count = bytes.next()?;
        let (list_bytes, actual_count) = read_entries(bytes, layout, list_count)?;
        (actual_count == list_count).then_some(Self {
            status,
            total_entries,
            start_index,
            list_count,
            list_bytes,
        })
    }

    /// Serialize the status and the applicable table header and entries.
    pub(super) fn encode(self) -> TableIter {
        let header = [
            self.status,
            self.total_entries,
            self.start_index,
            self.list_count,
        ];
        let header = if self.status == Status::Success as u8 {
            &header[..]
        } else {
            &header[..STATUS_BYTES]
        };

        #[cfg(feature = "alloc")]
        {
            let mut bytes = Vec::with_capacity(header.len() + self.list_bytes.len());
            bytes.extend_from_slice(header);
            bytes.extend(self.list_bytes);
            bytes.into_iter()
        }
        #[cfg(not(feature = "alloc"))]
        {
            Stream::new(header, self.list_bytes.into_iter())
        }
    }
}

/// Collect complete entries up to the wire count or the available storage capacity.
fn read_entries<I>(bytes: I, layout: EntryLayout, limit: u8) -> Option<(TableBytes, u8)>
where
    I: Iterator<Item = u8>,
{
    let mut bytes = bytes.peekable();
    let mut list = TableBytes::new();
    let mut count = u8::MIN;
    while bytes.peek().is_some() {
        if count == limit {
            return None;
        }
        layout.read(&mut bytes, &mut list)?;
        count += 1;
    }
    Some((list, count))
}

/// Append exactly one portion of an entry, accounting for allocation-free storage.
fn read_bytes<I>(bytes: &mut I, list: &mut TableBytes, length: usize) -> Option<()>
where
    I: Iterator<Item = u8>,
{
    for _ in 0..length {
        let byte = bytes.next()?;
        #[cfg(feature = "alloc")]
        list.push(byte);
        #[cfg(not(feature = "alloc"))]
        list.push(byte).ok()?;
    }
    Some(())
}

/// Define a management response using its entry layout and legacy raw payload fields.
macro_rules! management_table_response {
    (
        $(#[$attr:meta])*
        $response:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        layout: $layout:ident;
        payload: $payload:ty;
        fields: $total:ident, $list:ident;
    ) => {
        crate::zdp_command! {
            $(#[$attr])*
            $response => $name;
            cluster_id: $cluster_id;
            group: NetworkManagement;
            fields {
                table: super::table::Table,
            }
            constructor {
                /// Build a response from a raw table page or an error status.
                /// A malformed page produces an invalid-request response.
                #[must_use]
                pub fn new(response: Result<$payload, crate::Status>) -> Self {
                    let table = match response {
                        Ok(response) => super::table::Table::from_list(
                            response.$total,
                            response.start_index,
                            response.$list,
                            super::table::EntryLayout::$layout,
                        ),
                        Err(status) => super::table::Table::failed(status.into()),
                    };
                    Self { table }
                }
            }
            getters {
                /// Return the status, preserving unrecognized status codes as errors.
                ///
                /// # Errors
                /// Returns the raw code if it is unrecognized.
                pub fn status(&self) -> Result<crate::Status, u8> {
                    self.table.status.try_into()
                }

                /// Return the total number of entries on the device.
                #[must_use]
                pub const fn table_entries(&self) -> u8 {
                    self.table.total_entries
                }

                /// Return the starting index of this page.
                #[must_use]
                pub const fn start_index(&self) -> u8 {
                    self.table.start_index
                }

                /// Return the number of complete entries in this page.
                #[must_use]
                pub const fn list_count(&self) -> u8 {
                    self.table.list_count
                }

                /// Return the wire bytes of all entries in this page.
                #[must_use]
                pub fn list_bytes(&self) -> &[u8] {
                    &self.table.list_bytes
                }
            }
            le_stream {
                from {
                    fn from_le_stream<I>(bytes: I) -> Option<Self>
                    where
                        I: Iterator<Item = u8>,
                    {
                        let table = super::table::Table::decode(bytes, super::table::EntryLayout::$layout)?;
                        Some(Self { table })
                    }
                }
                to {
                    type Iter = super::table::TableIter;

                    fn to_le_stream(self) -> Self::Iter {
                        self.table.encode()
                    }
                }
            }
        }
    };
}

pub(super) use management_table_response;
