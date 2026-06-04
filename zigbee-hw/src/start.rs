use tokio::sync::mpsc::Receiver;

use crate::{Error, Event, NcpHandle};

/// Trait for starting an NCP driver.
pub trait Start {
    /// Start the NCP driver.
    fn start(self) -> impl Future<Output = Result<(NcpHandle, Receiver<Event>), Error>>;
}
