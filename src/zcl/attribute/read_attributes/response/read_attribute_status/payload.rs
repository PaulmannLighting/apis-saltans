use attribute_value::AttributeValue;

mod attribute_value;

pub struct Payload {
    data_type: u8,
    value: AttributeValue,
}
