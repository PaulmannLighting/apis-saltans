use crate::zcl::data_type::Class;

pub struct Structure {
    number_of_elements: u16,
    elements: Vec<Class>,
}
