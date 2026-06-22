use aps::Data;
use macaddr::MacAddr8;
use smarthomelib::{Event, EventReceiver};
use tokio::sync::mpsc::Receiver;
use zcl::{Cluster, Frame};

impl EventReceiver<MacAddr8, u8> for Receiver<Data<Frame<Cluster>>> {
    async fn recv(&mut self) -> Option<Event<MacAddr8, u8>> {
        let aps_frame = self.recv().await?;

        let (aps_header, zcl_frame) = aps_frame.into_parts();
        let (zcl_header, cluster) = zcl_frame.into_parts();

        match cluster {
            Cluster::OnOff(on_off) => Some(Event::new(aps_header.source, zcl_header.command_id)),
            _ => None,
        }
    }
}
