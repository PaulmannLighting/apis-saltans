use zb_core::ExpectResponse;

use crate::{Cluster, Command};

/// Whether a request permits a Default Response as its application-level outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefaultResponsePolicy {
    /// Require a command-specific response. Device rejections still terminate the request.
    SpecificRequired,

    /// Permit a Default Response, preserving its success, failure, or unknown status.
    DefaultAllowed,
}

/// Defines the response policy of a typed unicast request.
///
/// Commands with distinct response contracts use distinct request types.
/// The policy does not follow from the disable-default-response bit or the expected response type.
///
/// ```
/// use apis_saltans_zcl::{DefaultResponsePolicy, RequestPolicy};
/// use apis_saltans_zcl::global::read_attributes;
/// use apis_saltans_zcl::groups::RemoveAllGroups;
///
/// assert_eq!(
///     read_attributes::Command::DEFAULT_RESPONSE_POLICY,
///     DefaultResponsePolicy::SpecificRequired,
/// );
/// assert_eq!(
///     RemoveAllGroups::DEFAULT_RESPONSE_POLICY,
///     DefaultResponsePolicy::DefaultAllowed,
/// );
/// ```
pub trait RequestPolicy
where
    Self: Command + ExpectResponse<Cluster>,
{
    /// Whether this request type permits a Default Response as its application outcome.
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy;
}
