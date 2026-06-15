//! Zigbee transceiver API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transceiver regardless of the underlying hardware.

use core::time::Duration;

use const_env::env_item;

pub use self::api::{ColorControl, OnOff, ReadAttributeResult, ReadAttributes, WriteAttributes};
pub use self::coordinator::Coordinator;
pub use self::error::Error;
use crate::retry::Retry;

mod api;
mod binding;
mod coordinator;
mod discovery;
mod error;
mod expect;
mod mux;
mod network_manager;
mod retry;
mod timeout;
mod transceiver;

/// The maximum number of times to retry a Zigbee command.
#[env_item("ZIGBEE_COORDINATOR_MAX_RETRIES")]
const MAX_RETRIES: usize = 10;

/// The delay between retries, in seconds.
#[env_item("ZIGBEE_COORDINATOR_RETRY_DELAY_SECS")]
const RETRY_DELAY_SECS: u64 = 30;
const RETRY: Retry = Retry::new(MAX_RETRIES, Duration::from_secs(RETRY_DELAY_SECS));

/// The delay between retries, in seconds.
#[env_item("ZIGBEE_COORDINATOR_TASK_POOL_SIZE")]
const TASK_POOL_SIZE: usize = 16;

/// The delay between retries, in seconds.
#[env_item("ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE")]
const MPSC_CHANNEL_SIZE: usize = 128;
