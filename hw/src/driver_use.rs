#![cfg(feature = "driver-use")]

use std::pin::Pin;

use tokio::sync::mpsc::{Receiver, channel};
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
    fn init(self, events: Receiver<Event>) -> impl Future<Output = InitResult> + Send + 'static;

    /// Start the backend and prepare the support futures used to drive hardware events.
    ///
    /// The returned [`Futures`] contains the command handle, translated event receiver, and
    /// futures that must be polled by the caller to keep the hardware event path running.
    ///
    /// # Errors
    ///
    /// Returns an error if backend initialization fails.
    fn start(self, hw_events: Receiver<Self::HardwareEvent>) -> Result<Futures, Error> {
        let (msg_tx, msg_rx) = channel(hw_events.capacity());
        let (lib_events_tx, events) = channel(hw_events.capacity());
        let br = bridge(hw_events, msg_tx);
        let event_translator = Self::EventTranslator::new(lib_events_tx).run(msg_rx);
        let ncp: InitFuture = Box::pin(self.init(events));
        let dependencies: Vec<BoxedFuture<()>> = vec![Box::pin(br), Box::pin(event_translator)];
        Ok(Futures { ncp, dependencies })
    }
}

/// Running hardware support tasks and public handles.
pub struct Futures {
    /// Future that initializes the command actor and returns the public NCP handle and event stream.
    pub ncp: InitFuture,

    /// Futures that must be polled to keep backend event processing running.
    pub dependencies: Vec<BoxedFuture<()>>,
}
