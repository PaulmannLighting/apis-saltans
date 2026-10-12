use std::marker::PhantomData;

use bytes::Bytes;
use le_stream::ToLeStream;
use zb_aps::apsde::DataRequest;
use zb_core::ExpectResponse;
use zb_zcl::{
    Cluster, Command, Directed, RequestPolicy, Scoped, UnsequencedFrame, UnsequencedHeader,
};

use super::ZclResponseType;
use crate::zcl::ResponseExpectation;

/// An encoded ZCL request retaining its command's associated response contract.
///
/// Construct this from a typed command and explicit APS metadata, including the target
/// profile and cluster for global commands. The request type fixes its response contract;
/// that contract is erased only when the request is submitted to the actor.
/// Private fields prevent replacing the payload independently of its response contract.
///
/// Encoded payloads cannot be replaced independently of their response contracts:
///
/// ```compile_fail
/// use apis_saltans_coordinator::ZclRequest;
/// use zb_aps::apsde::DataRequest;
/// use zb_zcl::{UnsequencedFrame, groups::AddGroup};
/// use bytes::Bytes;
///
/// fn replace_payload(request: &mut ZclRequest<AddGroup>, encoded: DataRequest<UnsequencedFrame<Bytes>>) {
///     request.request = encoded;
/// }
/// ```
#[derive(Debug)]
pub struct ZclRequest<C> {
    request: DataRequest<UnsequencedFrame<Bytes>>,
    command: PhantomData<fn() -> C>,
}

impl<C> ZclRequest<C>
where
    C: Command + Directed + Scoped + ToLeStream + ExpectResponse<Cluster> + RequestPolicy,
    C::Response: ZclResponseType,
{
    /// Encode a command while preserving all APS request metadata and its response contract.
    pub fn new(request: DataRequest<C>) -> Self {
        Self {
            request: request.map_asdu(UnsequencedFrame::from_command),
            command: PhantomData,
        }
    }
}

impl<C> ZclRequest<C>
where
    C: RequestPolicy,
    C::Response: ZclResponseType + Send + Sync + 'static,
{
    /// Erase the type contract only when passing this request to the actor.
    pub(super) fn into_parts(self) -> (DataRequest<UnsequencedFrame<Bytes>>, ResponseExpectation) {
        (self.request, ResponseExpectation::for_request::<C>())
    }
}

impl<C> ZclRequest<C>
where
    C: Scoped,
{
    /// Inspect the encoded request without changing its response contract.
    #[must_use]
    pub const fn as_request(&self) -> &DataRequest<UnsequencedFrame<Bytes>> {
        &self.request
    }

    /// Set the outgoing Default Response flag without changing response correlation policy.
    #[must_use]
    pub fn with_disable_default_response(mut self, disabled: bool) -> Self {
        self.request = self
            .request
            .map_asdu(|frame| frame.with_disable_default_response(disabled));
        self
    }

    /// Override the manufacturer context, for example for manufacturer-specific attributes.
    #[must_use]
    pub fn with_manufacturer_code(mut self, manufacturer_code: Option<u16>) -> Self {
        self.request = self.request.map_asdu(|frame| {
            let (header, payload) = frame.into_parts();

            UnsequencedFrame::new(
                UnsequencedHeader::new(
                    C::SCOPE,
                    header.control().direction(),
                    header.control().disable_default_response(),
                    manufacturer_code,
                    header.command_id(),
                ),
                payload,
            )
        });
        self
    }
}
