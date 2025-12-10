/// Trait to add a tag onto a Type-Length-Value (TLV) structure.
pub trait Tlv {
    /// The TLV tag.
    const TAG: u8;
}
