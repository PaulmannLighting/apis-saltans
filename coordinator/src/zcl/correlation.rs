//! ZCL transaction identity, response matching, and lifecycle management.

use std::time::Duration;

use bytes::Bytes;
use log::debug;
use tokio::sync::mpsc::WeakSender;
use zb_aps::apsde::{DataIndication, DataRequest};
use zb_zcl::{Cluster, Frame, UnsequencedFrame};

pub use self::expectation::{ResponseExpectation, ResponseMatch};
use super::{Message, Transceiver};
use crate::Error;
use crate::aps::Aps;
use crate::correlation::{PROTOCOL_RESPONSE_TIMEOUT, Registry};
use crate::response::ApsProtocolResponse;
use crate::zcl::ZclKey;

mod expectation;
mod lifecycle;

/// Actor-owned ZCL transactions and their timeout/cancellation notification channel.
#[derive(Debug)]
pub(super) struct Responses {
    registry: Registry<ZclKey, Cluster, ResponseExpectation>,
    pub(super) response_timeout: Duration,
    inbox: WeakSender<Message>,
}

impl Responses {
    /// Create an empty set of response correlations.
    pub(super) const fn new(inbox: WeakSender<Message>) -> Self {
        Self {
            registry: Registry::new(),
            response_timeout: PROTOCOL_RESPONSE_TIMEOUT,
            inbox,
        }
    }

    /// Access transaction state when arranging actor regression tests.
    #[cfg(test)]
    pub(super) const fn registry(&mut self) -> &mut Registry<ZclKey, Cluster, ResponseExpectation> {
        &mut self.registry
    }

    /// Allocate a sequence for a request that must not solicit a unicast Default Response.
    pub(super) fn allocate_untracked(
        &mut self,
        request: &DataRequest<UnsequencedFrame<Bytes>>,
    ) -> Result<u8, Error> {
        let is_individual_unicast = ZclKey::from_request(request, u8::MIN).is_ok();
        if is_individual_unicast && !request.asdu().header().control().disable_default_response() {
            return Err(Error::ZclDefaultResponseEnabled);
        }
        self.registry
            .allocate_untracked_sequence(|sequence| ZclKey::from_request(request, sequence).ok())
    }

    /// Consume a matching response or late quarantined reply, leaving unrelated frames untouched.
    pub(super) fn handle_received(
        &mut self,
        indication: &DataIndication<Frame<Cluster>, (), ()>,
    ) -> bool {
        let Some(key) = ZclKey::from_indication(indication) else {
            return false;
        };
        let Some(expected) = self.registry.metadata(key) else {
            return false;
        };
        let result = match expected.classify(indication.asdu()) {
            ResponseMatch::Expected => Ok(indication.asdu().payload().clone()),
            ResponseMatch::Rejected(status) => Err(Error::Zcl(status)),
            ResponseMatch::UnexpectedDefault(command_id) => {
                Err(Error::UnexpectedDefaultResponse { command_id })
            }
            ResponseMatch::Unrelated => return false,
        };
        if self.registry.complete_result(key, result) {
            return true;
        }
        if self.registry.release_quarantine(key) {
            debug!(
                "Discarding late ZCL response with quarantined sequence {}",
                key.sequence
            );
            return true;
        }
        false
    }

    /// Send a ZCL unicast message with back-channel communication.
    ///
    /// # Returns
    ///
    /// Returns the response receiver.
    ///
    /// # Errors
    ///
    /// Returns an error if the unicast message could not be sent.
    pub(super) async fn communicate(
        &mut self,
        aps: &Aps,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        expected: ResponseExpectation,
    ) -> Result<ApsProtocolResponse<Cluster>, Error> {
        let (sequence_number, token, rx) = self.registry.try_register_with_metadata(
            |sequence| ZclKey::from_request(&request, sequence),
            expected,
        )?;
        self.schedule_response_timeout(token);

        let request = Transceiver::encode_request(request, sequence_number);

        let transmission = match aps.transmit(request).await {
            Ok(transmission) => transmission,
            Err(error) => {
                self.registry.discard(token);
                return Err(error);
            }
        };

        let cancellation = self.cancellation(token);

        Ok(ApsProtocolResponse::new(transmission, rx, cancellation))
    }
}

impl Drop for Responses {
    fn drop(&mut self) {
        self.registry.hardware_unavailable();
    }
}
