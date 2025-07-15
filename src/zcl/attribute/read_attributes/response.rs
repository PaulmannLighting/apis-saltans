use read_attribute_status::ReadAttributeStatus;

mod read_attribute_status;

/// Response to a read attributes command.
pub struct Response {
    attributes: Vec<ReadAttributeStatus>,
}
