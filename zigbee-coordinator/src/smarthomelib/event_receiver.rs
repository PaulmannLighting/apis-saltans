use log::warn;
use macaddr::MacAddr8;
use smarthomelib::{Event, EventReceiver};
use zigbee::Endpoint;

impl EventReceiver<MacAddr8, Endpoint> for crate::EventReceiver {
    async fn recv(&mut self) -> Option<Event<MacAddr8, Endpoint>> {
        let receiver = &mut **self;

        loop {
            if let Ok(event) = receiver.recv().await?.try_into().inspect_err(|error| {
                warn!("Failed to convert event: {error:?}");
            }) {
                return Some(event);
            }
        }
    }
}
