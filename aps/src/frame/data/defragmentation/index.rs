use apis_saltans_nwk::Source;

/// Key for an in-progress APS defragmentation transaction.
///
/// APS counters are scoped by source, so both the NWK source and APS counter
/// are required to distinguish transactions.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Index {
    source: Source,
    counter: u8,
}

impl Index {
    /// Create a new transaction index.
    #[must_use]
    pub const fn new(source: Source, counter: u8) -> Self {
        Self { source, counter }
    }
}
