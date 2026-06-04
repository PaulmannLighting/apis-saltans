use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zigbee_hw::Event;

/// The network management actor.
pub struct Actor {}

#[derive(Debug)]
pub struct Device {}

/// Messages received by the network management actor.
#[derive(Debug)]
pub enum Message {
    Event(Event),
    GetDevices {
        sender: oneshot::Sender<Box<[Device]>>,
    },
    Subscribe {
        sender: Sender<Box<[Device]>>,
    },
    DeviceUpdate {
        device: Device,
    },
}
