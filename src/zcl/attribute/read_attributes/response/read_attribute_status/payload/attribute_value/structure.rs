use crate::zcl::data_types::DataType;

pub struct Structure {
    number_of_elements: u16,
    elements: Vec<DataType>,
}
