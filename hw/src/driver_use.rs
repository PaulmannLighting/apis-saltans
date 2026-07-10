#![cfg(feature = "driver-use")]

use std::pin::Pin;

use tokio::sync::mpsc::{Receiver, Sender, channel};
use zb_zdp::SimpleDescriptor;

use crate::Error;
use crate::common::Event;
use crate::driver::{Backend, EventTranslator, bridge};

type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
type InitResult<T> = Result<(T, Receiver<Event>), Error>;
type InitFuture<T> = BoxedFuture<InitResult<T>>;

/// Builder for configuring a hardware backend and preparing its runtime futures.
///
/// Implement this trait for backend-specific builder types. The builder owns any configuration
/// derived from the coordinator's endpoint descriptors and turns a raw hardware event stream into a
/// [`Futures`] value. The caller is responsible for polling the returned futures in the documented
/// order.
pub trait Builder: Backend + Sized {
    /// Create a driver builder for the endpoints exposed by the coordinator.
    ///
    /// The endpoint descriptors describe the local Zigbee endpoints the coordinator will expose.
    /// Backend implementations can use them to configure NCP endpoint tables, application support
    /// sub-layer state, or other hardware-specific startup data.
    ///
    /// # Errors
    ///
    /// Returns an error if the backend cannot be configured for the supplied endpoint descriptors.
    fn new(endpoints: &[SimpleDescriptor]) -> Result<Self, Error>;

    /// Initialize the backend and return the driver plus translated event stream.
    ///
    /// `events` receives crate-level [`Event`] values produced by the event translator. The
    /// returned receiver is the event stream the caller should pass to coordinator code.
    ///
    /// `messages` is a sender connected to the event translator input. Backends can clone this
    /// sender and pass it to their command driver so driver operations can inject backend-specific
    /// translator messages when needed.
    ///
    /// The returned future must be `Send + 'static` because [`Futures::driver`] stores it as a
    /// boxed future that callers may spawn on a Tokio runtime.
    ///
    /// # Errors
    ///
    /// Returns an error if backend initialization fails.
    fn init(
        self,
        events: Receiver<Event>,
        messages: Sender<Self::Message>,
    ) -> impl Future<Output = InitResult<Self::Driver>> + Send + 'static;

    /// Prepare the futures needed to run the backend.
    ///
    /// This method wires the raw hardware event stream into the backend translator and returns a
    /// [`Futures`] value containing:
    ///
    /// - [`Futures::dependencies`], which bridge and translate hardware events.
    /// - [`Futures::driver`], which runs [`Self::init`] and yields the initialized driver plus the
    ///   translated event receiver.
    ///
    /// Spawn or otherwise poll every dependency future before spawning or awaiting
    /// [`Futures::driver`].
    ///
    /// # Errors
    ///
    /// Returns an error if the runtime futures cannot be prepared.
    fn start(
        self,
        hw_events: Receiver<Self::HardwareEvent>,
    ) -> Result<Futures<Self::Driver>, Error> {
        let (msg_tx, msg_rx) = channel(hw_events.capacity());
        let (lib_events_tx, events) = channel(hw_events.capacity());
        let br = bridge(hw_events, msg_tx.clone());
        let event_translator = Self::EventTranslator::new(lib_events_tx).run(msg_rx);
        let driver: InitFuture<Self::Driver> = Box::pin(self.init(events, msg_tx));
        let dependencies: Vec<BoxedFuture<()>> = vec![Box::pin(br), Box::pin(event_translator)];
        Ok(Futures {
            driver,
            dependencies,
        })
    }
}

/// Runtime futures for a configured hardware backend.
///
/// `Futures` is returned by [`Builder::start`]. It intentionally separates support tasks from the
/// backend initialization future so the caller can start the event pipeline first.
///
/// Polling order matters:
///
/// 1. Spawn or otherwise poll every future in [`Self::dependencies`].
/// 2. Spawn or await [`Self::driver`].
///
/// Starting `driver` first can leave backend initialization waiting for event infrastructure that is
/// not running yet.
pub struct Futures<T> {
    /// Future that initializes the backend driver and returns it with the translated event stream.
    ///
    /// The returned driver is the backend's [`Backend::Driver`] type. The returned receiver emits
    /// crate-level [`Event`] values and is intended to be passed to coordinator startup code.
    ///
    /// Spawn or await this only after all [`Self::dependencies`] are already being polled.
    pub driver: InitFuture<T>,

    /// Futures that must be polled before [`Self::driver`] starts.
    ///
    /// These futures keep backend event processing running, including the bridge from raw hardware
    /// events into translator messages and the translator that emits crate-level events.
    pub dependencies: Vec<BoxedFuture<()>>,
}
