use super::array::Array;
use super::structure::Structure;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrderedSequence {
    Array(Array),
    Structure(Structure),
}
