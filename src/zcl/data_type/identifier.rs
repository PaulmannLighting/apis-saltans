/// Identifier for a cluster, attribute, or BACnet OID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Identifier {
    ClusterId(u16),
    AttributeId(u16),
    BacNetOid(u32),
}
