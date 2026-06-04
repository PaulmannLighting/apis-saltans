use zigbee_hw::Event;

use crate::network_manager::Device;

/// The binding management actor.
pub struct Actor {}

/// Messages received by the binding management actor.
#[derive(Debug)]
pub enum Message {
    Event(Event),
    Device(Device),
}
