use read_attribute_status::ReadAttributeStatus;

use crate::types::U8Vec;

mod read_attribute_status;

/// Response to a read attribute command.
pub struct Response {
    attributes: U8Vec<ReadAttributeStatus>,
}
