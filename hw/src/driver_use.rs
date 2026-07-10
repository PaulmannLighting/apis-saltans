#![cfg(feature = "driver-use")]

use std::pin::Pin;

use tokio::sync::mpsc::{Receiver, Sender, channel};
use zb_zdp::SimpleDescriptor;

use crate::common::Event;
use crate::driver::{Backend, EventTranslator, bridge};
use crate::{Error, NcpHandle};

type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
type InitResult = Result<(NcpHandle, Receiver<Event>), Error>;
type InitFuture = BoxedFuture<InitResult>;

/// Constructs and prepares a configured hardware backend.
pub trait Builder: Backend + Sized {
    /// Create a driver builder for the endpoints exposed by the coordinator.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend cannot be configured for the supplied endpoint descriptors.
    fn new(endpoints: &[SimpleDescriptor]) -> Result<Self, Error>;

    /// Initialize the backend command side.
    ///
    /// The `events` receiver contains crate-level events produced by the event translator. The
    /// returned receiver is the event stream the caller should pass to coordinator code.
    ///
    /// # Errors
    ///
    /// Returns an error if backend initialization fails.
    fn init(
        self,
        events: Receiver<Event>,
        messages: Sender<Self::Message>,
    ) -> impl Future<Output = InitResult> + Send + 'static;

    /// Start the backend and prepare the futures needed to run it.
    ///
    /// The returned [`Futures`] separates the backend initialization future from the dependency
    /// futures that feed it translated events. Spawn or otherwise poll every dependency future
    /// before spawning or awaiting [`Futures::ncp`].
    ///
    /// # Errors
    ///
    /// Returns an error if the runtime futures cannot be prepared.
    fn start(self, hw_events: Receiver<Self::HardwareEvent>) -> Result<Futures, Error> {
        let (msg_tx, msg_rx) = channel(hw_events.capacity());
        let (lib_events_tx, events) = channel(hw_events.capacity());
        let br = bridge(hw_events, msg_tx.clone());
        let event_translator = Self::EventTranslator::new(lib_events_tx).run(msg_rx);
        let ncp: InitFuture = Box::pin(self.init(events, msg_tx));
        let dependencies: Vec<BoxedFuture<()>> = vec![Box::pin(br), Box::pin(event_translator)];
        Ok(Futures { ncp, dependencies })
    }
}

/// Futures required to run a configured hardware backend.
///
/// The dependency futures drive the event path into the backend initialization future. Callers must
/// spawn or otherwise poll all [`Self::dependencies`] before spawning or awaiting [`Self::ncp`].
/// Starting `ncp` first can leave backend initialization waiting for event infrastructure that is
/// not running yet.
pub struct Futures {
    /// Future that initializes the command actor and returns the public NCP handle and event stream.
    ///
    /// Spawn or await this only after all [`Self::dependencies`] are already being polled.
    pub ncp: InitFuture,

    /// Futures that must be polled before [`Self::ncp`] starts.
    ///
    /// These futures keep backend event processing running, including the bridge from raw hardware
    /// events into translator messages and the translator that emits crate-level events.
    pub dependencies: Vec<BoxedFuture<()>>,
}
