use bytes::Bytes;
use le_stream::FromLeStream;
use log::{debug, trace, warn};
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::oneshot::Receiver;
use zb_aps::apsde::{DataIndication, DataRequest};
use zb_zcl::global::default_response::DefaultResponse;
use zb_zcl::global::report_attributes::Command as ReportAttributes;
use zb_zcl::{Cluster, Command, Frame, Header, ParseFrameError, Scope, UnsequencedFrame};

use super::{SubscriptionMessage, Transceiver};
use crate::correlation::{Key, Token};
use crate::response::ApsProtocolResponse;
use crate::{Error, Event, RawExpectedPacket};

/// Matching policy follows the reserved TSN into quarantine, independently of its response sink.
#[derive(Debug)]
pub(super) enum ExpectedResponse {
    Typed,
    Raw {
        packet: RawExpectedPacket,
        command_id: u8,
    },
}

impl ExpectedResponse {
    const fn is_typed(&self) -> bool {
        matches!(self, Self::Typed)
    }

    fn matches_raw(&self, header: Header, first_body_byte: Option<u8>) -> bool {
        let Self::Raw { packet, command_id } = self else {
            return false;
        };
        header.control().typ() == Ok(packet.scope())
            && header.command_id() == packet.command_id()
            && (packet.scope() != Scope::Global
                || packet.command_id() != DefaultResponse::ID
                || first_body_byte == Some(*command_id))
    }
}

/// Inbound response correlation precedes typed subscriptions and application events.
impl Transceiver {
    pub(super) fn handle_message_received(&mut self, indication: DataIndication<Bytes, (), ()>) {
        let mut bytes = indication.asdu().iter().copied();
        let header = Header::from_le_stream(&mut bytes)
            .ok_or(ParseFrameError::MissingHeader)
            .and_then(|header| {
                header
                    .control()
                    .typ()
                    .map(|_| header)
                    .map_err(ParseFrameError::InvalidType)
            });
        let header = match header {
            Ok(header) => header,
            Err(error) => {
                warn!("Failed to parse ZCL data indication: {error}");
                return;
            }
        };
        let source = indication.metadata().source();
        let Some(key) = Key::from_received_zcl_header(indication.metadata(), header) else {
            warn!("Discarding ZCL indication from unsupported source: {source:?}");
            return;
        };
        let first_body_byte = bytes.next();
        let is_report = header.control().typ() == Ok(Scope::Global)
            && header.command_id() == ReportAttributes::ID;

        // A valid header suffices for raw delivery; preserve the complete original ASDU.
        if !is_report {
            if self
                .responses
                .complete_raw(key, indication.asdu().clone(), |expected| {
                    expected.matches_raw(header, first_body_byte)
                })
            {
                return;
            }
            if self.responses.release_quarantine_matching(key, |expected| {
                expected.matches_raw(header, first_body_byte)
            }) {
                debug!(
                    "Discarding late raw ZCL response with quarantined sequence {}",
                    key.sequence()
                );
                return;
            }
        }

        let (metadata, asdu) = indication.into_parts();
        let frame = match Frame::parse(metadata.cluster_id(), asdu.into_iter()) {
            Ok(frame) => frame,
            Err(error) => {
                warn!("Failed to parse ZCL data indication: {error}");
                return;
            }
        };
        let indication = DataIndication::new(metadata, frame);
        trace!("Received ZCL message from {source:?}: {indication:?}");
        if !is_report
            && self.responses.complete_matching(
                key,
                indication.asdu().payload().clone(),
                ExpectedResponse::is_typed,
            )
        {
            return;
        }
        if !is_report
            && self
                .responses
                .release_quarantine_matching(key, ExpectedResponse::is_typed)
        {
            debug!(
                "Discarding late ZCL response with quarantined sequence {}",
                key.sequence()
            );
            return;
        }
        if !self.forward_to_subscribers(&indication) {
            self.events.emit(Event::Zcl { indication });
        }
    }

    /// Deliver a received frame to every matching live subscription.
    fn forward_to_subscribers(
        &mut self,
        indication: &DataIndication<Frame<Cluster>, (), ()>,
    ) -> bool {
        let mut delivered = false;
        self.subscriptions.retain(|subscription| {
            if !subscription.is_open() {
                return false;
            }
            if !subscription.matches(indication) {
                return true;
            }
            let message = SubscriptionMessage {
                indication: indication.clone(),
            };
            match subscription.try_send(message) {
                Ok(()) => {
                    delivered = true;
                    true
                }
                Err(TrySendError::Full(_)) => {
                    warn!("ZCL subscription channel is full; forwarding frame to normal routing");
                    true
                }
                Err(TrySendError::Closed(_)) => false,
            }
        });
        delivered
    }
}

/// Both response kinds register before APS handoff and use the same deferred lifecycle.
impl Transceiver {
    pub(super) async fn communicate(
        &mut self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
    ) -> Result<ApsProtocolResponse<Cluster>, Error> {
        let (sequence, token, response) = self.responses.try_register_matching(
            |sequence| Self::request_key(&request, sequence),
            ExpectedResponse::Typed,
        )?;
        self.communicate_registered(request, sequence, token, response)
            .await
    }

    pub(super) async fn communicate_raw(
        &mut self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        expected_response: RawExpectedPacket,
    ) -> Result<ApsProtocolResponse<Bytes>, Error> {
        let matcher = ExpectedResponse::Raw {
            packet: expected_response,
            command_id: request.asdu().header().command_id(),
        };
        let (sequence, token, response) = self
            .responses
            .try_register_raw(|sequence| Self::request_key(&request, sequence), matcher)?;
        self.communicate_registered(request, sequence, token, response)
            .await
    }

    async fn communicate_registered<T>(
        &mut self,
        request: DataRequest<UnsequencedFrame<Bytes>>,
        sequence: u8,
        token: Token,
        response: Receiver<Result<T, Error>>,
    ) -> Result<ApsProtocolResponse<T>, Error> {
        self.schedule_response_timeout(token);
        let request = Self::encode_request(request, sequence);
        let transmission = match self.aps.transmit(request).await {
            Ok(transmission) => transmission,
            Err(error) => {
                self.responses.discard(token);
                return Err(error);
            }
        };
        Ok(ApsProtocolResponse::new(
            transmission,
            response,
            self.cancellation(token),
        ))
    }
}
