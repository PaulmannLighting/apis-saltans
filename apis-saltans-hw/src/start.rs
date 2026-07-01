use apis_saltans_zdp::SimpleDescriptor;
use tokio::sync::mpsc::Receiver;

use crate::{Error, Event, NcpHandle};

/// Trait for starting an NCP driver.
pub trait Start {
    /// Start the NCP driver.
    fn start(
        self,
        endpoints: &[SimpleDescriptor],
    ) -> impl Future<Output = Result<(NcpHandle, Receiver<Event>), Error>>;
}
