#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Collection {
    Set(Vec<u8>),
    Bag(Vec<u8>),
}
