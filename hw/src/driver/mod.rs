#![cfg(feature = "driver")]

//! Implementor-facing traits and helpers for Zigbee NCP driver backends.

pub use driver::Driver;

pub use self::backend::Backend;
pub use self::bridge::bridge;
pub use self::event_translator::EventTranslator;

mod backend;
mod bridge;
#[expect(clippy::module_inception)]
mod driver;
mod event_translator;
