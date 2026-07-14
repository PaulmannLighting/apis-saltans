/// A value type associated with a Zigbee data type identifier.
pub trait TypeId {
    /// The Zigbee data type identifier.
    const ID: u8;
}
