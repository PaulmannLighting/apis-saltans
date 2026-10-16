#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};
use core::num::NonZero;

use log::warn;

use super::AssemblyError;
use crate::Payload;
use crate::data::{Frame, Header};
#[cfg(not(feature = "alloc"))]
use crate::{CapacityError, CapacityKind, FRAGMENT_CAPACITY, FRAGMENTS_CAPACITY, PAYLOAD_CAPACITY};

#[cfg(feature = "alloc")]
type Fragment = Payload;
#[cfg(not(feature = "alloc"))]
type Fragment = heapless::Vec<u8, FRAGMENT_CAPACITY>;
#[cfg(feature = "alloc")]
type Slots = Vec<Option<Fragment>>;
#[cfg(not(feature = "alloc"))]
type Slots = heapless::Vec<Option<Fragment>, FRAGMENTS_CAPACITY>;

/// Buffered fragments for one APS defragmentation transaction.
#[derive(Debug)]
pub struct Transaction {
    header: Header,
    fragments: Slots,
}

impl Transaction {
    /// Allocate the advertised slots and store the first fragment.
    pub fn new(
        length: NonZero<u8>,
        header: Header,
        first_fragment: Payload,
    ) -> Result<Self, AssemblyError> {
        #[cfg(feature = "alloc")]
        let fragments = vec![None; usize::from(length.get())];
        #[cfg(not(feature = "alloc"))]
        let fragments = {
            let mut slots = Slots::new();
            slots
                .resize(usize::from(length.get()), None)
                .map_err(|_| CapacityError {
                    kind: CapacityKind::Fragments,
                    required: usize::from(length.get()),
                    capacity: FRAGMENTS_CAPACITY,
                })?;
            slots
        };
        let mut result = Self { header, fragments };
        result.store(0, first_fragment)?;
        Ok(result)
    }

    #[cfg_attr(
        not(feature = "alloc"),
        expect(
            clippy::needless_pass_by_value,
            reason = "the shared storage interface consumes Bytes in allocating mode"
        )
    )]
    fn store(&mut self, index: u8, fragment: Payload) -> Result<(), AssemblyError> {
        let index = usize::from(index);
        if index >= self.fragments.len() {
            return Err(AssemblyError::InvalidFragment);
        }
        #[cfg(not(feature = "alloc"))]
        let fragment = {
            let size = self
                .fragments
                .iter()
                .enumerate()
                .filter(|(position, _)| *position != index)
                .filter_map(|(_, slot)| slot.as_ref())
                .fold(fragment.len(), |size, bytes| {
                    size.saturating_add(bytes.len())
                });
            if size > PAYLOAD_CAPACITY {
                return Err(CapacityError {
                    kind: CapacityKind::Payload,
                    required: size,
                    capacity: PAYLOAD_CAPACITY,
                }
                .into());
            }
            Fragment::from_slice(&fragment).map_err(|_| CapacityError {
                kind: CapacityKind::Fragment,
                required: fragment.len(),
                capacity: FRAGMENT_CAPACITY,
            })?
        };
        if self.fragments[index].replace(fragment).is_some() {
            warn!("Duplicate fragment for index: {index}");
        }
        Ok(())
    }

    /// Insert a fragment; invalid or excessive fragments discard the consumed transaction.
    pub fn insert(mut self, index: u8, fragment: Payload) -> Result<InsertResult, AssemblyError> {
        self.store(index, fragment)?;
        if self.fragments.iter().all(Option::is_some) {
            self.header.drop_extended();
            let bytes = self.fragments.into_iter().flatten().flatten();
            #[cfg(feature = "alloc")]
            let payload = bytes.collect();
            #[cfg(not(feature = "alloc"))]
            let payload = Payload::try_from_iter(bytes)?;
            return Ok(InsertResult::Complete(Frame::raw(self.header, payload)));
        }
        Ok(InsertResult::Incomplete(self))
    }
}

/// Result of inserting a valid fragment into a transaction.
#[derive(Debug)]
#[cfg_attr(
    not(feature = "alloc"),
    allow(
        clippy::large_enum_variant,
        reason = "bounded mode stores transactions inline without a heap"
    )
)]
pub enum InsertResult {
    /// All fragments are present and the APS frame has been rebuilt.
    Complete(Frame<Payload>),

    /// The transaction still needs more fragments.
    Incomplete(Transaction),
}
