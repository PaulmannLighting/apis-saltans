use tokio::sync::mpsc::Receiver;

use crate::initialize::Initialize;
use crate::{Error, Event, NcpHandle};

/// Prepared driver tasks that have not yet been started.
pub struct PreparedHardware<Builder, Bridge, Translator> {
    pub(crate) builder: Builder,
    pub(crate) events: Receiver<Event>,
    pub(crate) bridge: Bridge,
    pub(crate) translator: Translator,
}
impl<Builder, Bridge, Translator> PreparedHardware<Builder, Bridge, Translator>
where
    Builder: Initialize,
    Bridge: Future<Output = ()> + Send + 'static,
    Translator: Future<Output = ()> + Send + 'static,
{
    /// Spawn the bridge and translator tasks, initialize the driver, and return its public handles.
    ///
    /// # Errors
    ///
    /// Returns an error if driver initialization fails.
    pub async fn start(self) -> Result<(NcpHandle, Receiver<Event>), Error> {
        tokio::spawn(self.bridge);
        tokio::spawn(self.translator);
        let ncp = self.builder.init().await?;
        Ok((ncp, self.events))
    }
}
