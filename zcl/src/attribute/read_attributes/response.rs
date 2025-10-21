use read_attribute_status::ReadAttributeStatus;
use zb::constants::U8_CAPACITY;

mod read_attribute_status;

/// Response to a read attribute command.
pub struct Response<const CAPACITY: usize = U8_CAPACITY> {
    attributes: heapless::Vec<ReadAttributeStatus, CAPACITY>,
}
