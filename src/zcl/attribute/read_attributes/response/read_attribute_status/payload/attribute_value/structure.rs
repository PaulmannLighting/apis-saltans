use crate::zcl::data_type::DataType;

pub struct Structure {
    number_of_elements: u16,
    elements: Vec<DataType>,
}
