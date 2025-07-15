use crate::zcl::data_types::DataType;

pub struct ArraySetBag {
    element_type: u8,
    number_of_elements: u16,
    elements: Vec<DataType>,
}
