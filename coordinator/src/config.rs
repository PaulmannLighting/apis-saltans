//! Runtime-selectable coordinator settings.

use std::num::{NonZero, NonZeroUsize};
use std::time::Duration;

use const_env::env_lit;

use crate::correlation::PROTOCOL_RESPONSE_TIMEOUT;
use crate::{DEFAULT_OTA_UPDATE_TASK_LIMIT, MPSC_CHANNEL_SIZE};

const CONFIRMATION_TIMEOUT_SECS: NonZero<u64> = NonZero::new(env_lit!(
    "ZIGBEE_COORDINATOR_APS_CONFIRMATION_TIMEOUT_SECS",
    30
))
.expect("ZIGBEE_COORDINATOR_APS_CONFIRMATION_TIMEOUT_SECS must be greater than zero");

/// Default time to await an APS confirmation after backend acceptance.
///
/// Set at compile time by `ZIGBEE_COORDINATOR_APS_CONFIRMATION_TIMEOUT_SECS`;
/// defaults to 30 seconds and must be greater than zero.
pub const DEFAULT_CONFIRMATION_TIMEOUT: Duration =
    Duration::from_secs(CONFIRMATION_TIMEOUT_SECS.get());

/// Settings fixed for the lifetime of one coordinator runtime.
///
/// Defaults use the build-time inbox, protocol response, and APS confirmation timeout settings.
/// Deadlines may be zero for immediate expiry. Late-response quarantine remains an independent
/// internal safeguard and is not shortened by these deadlines.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoordinatorConfig {
    /// Capacity of protocol actor inboxes, OTA transfer inboxes, and OTA subscriptions.
    pub inbox_capacity: NonZeroUsize,
    /// Maximum simultaneous destination OTA transfers; zero rejects all offers.
    pub ota_update_task_limit: usize,
    /// Maximum simultaneous ZDP requests being handed to APS; zero rejects new submissions.
    pub zdp_submission_limit: usize,
    /// Maximum simultaneous inbound ZDP server operations; zero ignores new server requests.
    pub zdp_server_operation_limit: usize,
    /// Time from response registration until a ZCL or ZDP response deadline expires.
    pub protocol_response_timeout: Duration,
    /// Time from backend acceptance until an acknowledged APS transmission must be confirmed.
    pub aps_confirmation_timeout: Duration,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            inbox_capacity: NonZeroUsize::new(MPSC_CHANNEL_SIZE)
                .expect("coordinator inbox capacity must be nonzero"),
            ota_update_task_limit: DEFAULT_OTA_UPDATE_TASK_LIMIT,
            zdp_submission_limit: MPSC_CHANNEL_SIZE,
            zdp_server_operation_limit: MPSC_CHANNEL_SIZE,
            protocol_response_timeout: PROTOCOL_RESPONSE_TIMEOUT,
            aps_confirmation_timeout: DEFAULT_CONFIRMATION_TIMEOUT,
        }
    }
}
