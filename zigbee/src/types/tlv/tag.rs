/// Trait to add a tag onto a Type-Length-Value (TLV) structure.
pub trait Tag {
    /// The TLV tag.
    const TAG: u8;

    /// Get the amount of bytes in the TLV data structure.
    fn size(&self) -> usize;

    /// Get the total serialized size of the TLV structure, including tag and length.
    fn serialized_size(&self) -> u8 {
        u8::try_from(
            self.size()
                .checked_sub(1)
                .expect("Size is guaranteed to be at least 1"),
        )
        .expect("Size fits in u8")
    }
}
