use apis_saltans_nwk::Sender;

/// Key for an in-progress APS defragmentation transaction.
///
/// APS counters are scoped by sender, so both the NWK sender and APS counter
/// are required to distinguish transactions.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Index {
    sender: Sender,
    counter: u8,
}

impl Index {
    /// Create a new transaction index.
    #[must_use]
    pub const fn new(sender: Sender, counter: u8) -> Self {
        Self { sender, counter }
    }
}
