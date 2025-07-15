use array_set_bag::ArraySetBag;
use structure::Structure;

use crate::zcl::data_type::DataType;

mod array_set_bag;
mod structure;

pub enum AttributeValue {
    Array(ArraySetBag),
    Set(ArraySetBag),
    Bag(ArraySetBag),
    Structure(Structure),
    Simple(DataType),
}
