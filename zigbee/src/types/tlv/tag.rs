/// Trait to add a tag onto a Type-Length-Value (TLV) structure.
pub trait Tag {
    /// The TLV tag.
    const TAG: u8;
}
