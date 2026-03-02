use zigbee::Cluster;

/// A trait to allow the reading of attributes by their respective IDs in a type-safe manner.
pub trait ReadableAttribute: Cluster {
    /// The type of attribute, usually an enum, which is returned from the read.
    type ReadAttribute;
}
