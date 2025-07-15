use array_set_bag::ArraySetBag;
use structure::Structure;

mod array_set_bag;
mod structure;

pub enum AttributeValue {
    Array(ArraySetBag),
    Set(ArraySetBag),
    Bag(ArraySetBag),
    Structure(Structure),
}
