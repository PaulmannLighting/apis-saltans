//! Cancellation, network failures, and timeout scheduling for ZCL transactions.

use super::Responses;
use crate::correlation::{
    Cancellation, PROTOCOL_QUARANTINE_TIMEOUT, PROTOCOL_RESPONSE_TIMEOUT, Token, cancellation,
    schedule_timeout,
};
use crate::zcl::Message;

impl Responses {
    /// Fail pending responses at a network boundary.
    pub(in crate::zcl) fn network_down(&mut self) {
        self.registry
            .network_down(&zb_hw::TransmissionError::NoRoute);
    }

    /// Fail all pending responses when the hardware event stream closes.
    pub(in crate::zcl) fn hardware_unavailable(&mut self) {
        self.registry.hardware_unavailable();
    }

    /// Quarantine a cancelled transaction and schedule its release.
    pub(in crate::zcl) fn cancel(&mut self, token: Token) {
        if self.registry.cancel(token) {
            self.schedule_quarantine_timeout(token);
        }
    }

    /// Expire a pending transaction and schedule release of its identity.
    pub(in crate::zcl) fn timeout(&mut self, token: Token) {
        if self.registry.timeout(token) {
            self.schedule_quarantine_timeout(token);
        }
    }

    /// Release a quarantined identity when its grace period expires.
    pub(in crate::zcl) fn expire_quarantine(&mut self, token: Token) {
        self.registry.expire_quarantine(token);
    }

    /// Create a drop guard that notifies the actor when the caller abandons a response.
    pub(super) fn cancellation(&self, token: Token) -> Cancellation {
        cancellation(
            self.inbox.clone(),
            token,
            |token| Message::Cancel { token },
            "ZCL",
        )
    }

    /// Schedule the response deadline without keeping the actor alive.
    pub(super) fn schedule_response_timeout(&self, token: Token) {
        schedule_timeout(
            self.inbox.clone(),
            PROTOCOL_RESPONSE_TIMEOUT,
            Message::ResponseTimeout { token },
            "ZCL response timeout",
        );
    }

    fn schedule_quarantine_timeout(&self, token: Token) {
        schedule_timeout(
            self.inbox.clone(),
            PROTOCOL_QUARANTINE_TIMEOUT,
            Message::QuarantineTimeout { token },
            "ZCL quarantine timeout",
        );
    }
}
