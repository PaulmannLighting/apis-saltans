use zigbee::Cluster;

mod raw;
pub mod readable;

/// A trait to allow the reading of attributes by their respective IDs in a type-safe manner.
pub trait ReadableAttribute: Copy + Into<u16> + Cluster {
    /// The type of attribute, usually an enum, which is returned from the readable.
    type ReadAttribute;
}
