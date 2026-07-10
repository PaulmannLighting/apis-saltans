#![cfg(feature = "driver-use")]

//! An interface for communicating with a Zigbee NCP (Network Co-Processor) device.

#[cfg(feature = "driver")]
pub use driver::Driver;

pub use self::backend::Backend;
pub use self::bridge::bridge;
pub use self::event_translator::EventTranslator;
pub use self::initialize::Initialize;

mod backend;
mod bridge;
#[expect(clippy::module_inception)]
mod driver;
mod event_translator;
mod initialize;
