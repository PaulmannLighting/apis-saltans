use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zcl::Cluster;
use zigbee_hw::Event;

/// Messages received by the multiplexer.
#[derive(Debug)]
pub enum Message {
    /// An event from the hardware layer.
    Event(Event),
    /// Subscribe to the response multiplexer.
    SubscribeZclResponse {
        /// ZCL sequence number.
        seq: u8,
        /// ZCL response channel.
        response: oneshot::Sender<Cluster>,
    },
    /// Subscribe to any kind of event.
    SubscribeEvent {
        /// The sender to send to.
        sender: Sender<Event>,
    },
}
