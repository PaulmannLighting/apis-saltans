use std::num::NonZero;

use log::warn;

use crate::data::{Frame, Header};

/// Buffered fragments for one APS defragmentation transaction.
///
/// The first fragment provides the authoritative APS header and the expected
/// fragment count. Follow-up fragments are stored by their block index until all
/// slots have been filled.
#[derive(Debug)]
pub struct Transaction {
    header: Header,
    fragments: Vec<Option<Box<[u8]>>>,
}

impl Transaction {
    /// Start a transaction from the first fragment.
    ///
    /// `length` is the total number of fragments expected for the APS frame.
    /// The first fragment is inserted into slot `0`.
    #[must_use]
    pub fn new(length: NonZero<u8>, header: Header, first_fragment: Box<[u8]>) -> Self {
        let mut fragments = vec![None; length.get().into()];
        fragments[0].replace(first_fragment);
        Self { header, fragments }
    }

    /// Insert a follow-up fragment into the transaction.
    ///
    /// If the fragment completes the transaction, the returned frame contains
    /// the concatenated payload and the saved header with its extended header
    /// removed.
    pub fn insert(mut self, index: u8, fragment: Box<[u8]>) -> InsertResult {
        let Some(slot) = self.fragments.get_mut(usize::from(index)) else {
            return InsertResult::OutOfBounds(index);
        };

        if slot.replace(fragment).is_some() {
            warn!("Duplicate fragment for index: {index}");
        }

        if self.fragments.iter().all(Option::is_some) {
            self.header.drop_extended();
            return InsertResult::Complete(Frame::raw(
                self.header,
                self.fragments
                    .into_iter()
                    .flat_map(|opt| opt.expect("All of the options are Some()."))
                    .collect(),
            ));
        }

        InsertResult::Incomplete(self)
    }
}

/// Result of inserting a fragment into a transaction.
#[derive(Debug)]
pub enum InsertResult {
    /// All fragments are present and the APS frame has been rebuilt.
    Complete(Frame<Box<[u8]>>),

    /// The transaction still needs more fragments.
    Incomplete(Transaction),

    /// The fragment index was outside the expected fragment range.
    OutOfBounds(u8),
}
