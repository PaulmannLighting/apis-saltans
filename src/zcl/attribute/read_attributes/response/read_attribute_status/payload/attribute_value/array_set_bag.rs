use crate::zcl::data_type::Class;

pub struct ArraySetBag {
    element_type: u8,
    number_of_elements: u16,
    elements: Vec<Class>,
}
