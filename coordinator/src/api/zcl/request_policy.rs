use zb_core::ExpectResponse;
use zb_zcl::global::{configure_reporting, read_attributes, write_attributes};
use zb_zcl::groups::{
    AddGroup, AddGroupIfIdentifying, GetGroupMembership, RemoveAllGroups, RemoveGroup, ViewGroup,
};
use zb_zcl::ota_upgrade::{UpgradeComplete, UpgradeIncomplete};
use zb_zcl::{Cluster, Command};

use super::DefaultResponsePolicy;

/// Defines the response policy of a typed unicast request.
///
/// Commands with distinct response contracts use distinct request types.
/// The policy does not follow from the disable-default-response bit or the expected response type.
pub trait ZclRequestPolicy: Command + ExpectResponse<Cluster> {
    /// Whether this request type permits a Default Response as its application outcome.
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy;
}

impl ZclRequestPolicy for AddGroup {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl ZclRequestPolicy for RemoveGroup {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl ZclRequestPolicy for GetGroupMembership {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl ZclRequestPolicy for read_attributes::Command {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl ZclRequestPolicy for write_attributes::Command {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl ZclRequestPolicy for configure_reporting::Send {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl ZclRequestPolicy for UpgradeComplete {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl ZclRequestPolicy for UpgradeIncomplete {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::DefaultAllowed;
}

impl ZclRequestPolicy for ViewGroup {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::SpecificRequired;
}

impl ZclRequestPolicy for RemoveAllGroups {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::DefaultAllowed;
}

impl ZclRequestPolicy for AddGroupIfIdentifying {
    const DEFAULT_RESPONSE_POLICY: DefaultResponsePolicy = DefaultResponsePolicy::DefaultAllowed;
}
