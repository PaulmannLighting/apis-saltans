use zb_zcl::global::{configure_reporting, read_attributes, write_attributes};
use zb_zcl::groups::{
    AddGroup, AddGroupIfIdentifying, GetGroupMembership, RemoveAllGroups, RemoveGroup, ViewGroup,
};
use zb_zcl::ota_upgrade::{UpgradeEndRequest, UpgradeEndStatus};

use super::DefaultResponsePolicy;

/// Derives response policy from a typed unicast request before its payload is encoded.
///
/// Implementations must follow the command's response rules, including payload-dependent cases.
/// The policy does not follow from the disable-default-response bit or the expected response type.
pub trait ZclRequestPolicy {
    /// Whether this particular request permits a Default Response as its application outcome.
    fn default_response_policy(&self) -> DefaultResponsePolicy;
}

impl ZclRequestPolicy for AddGroup {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::SpecificRequired
    }
}

impl ZclRequestPolicy for RemoveGroup {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::SpecificRequired
    }
}

impl ZclRequestPolicy for GetGroupMembership {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::SpecificRequired
    }
}

impl ZclRequestPolicy for read_attributes::Command {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::SpecificRequired
    }
}

impl ZclRequestPolicy for write_attributes::Command {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::SpecificRequired
    }
}

impl ZclRequestPolicy for configure_reporting::Send {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::SpecificRequired
    }
}

impl ZclRequestPolicy for UpgradeEndRequest {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        // ZCL revision 8, section 11.13.9.4: non-success download outcomes receive
        // a Default Response instead of an Upgrade End Response.
        if self.status() == UpgradeEndStatus::Success {
            DefaultResponsePolicy::SpecificRequired
        } else {
            DefaultResponsePolicy::DefaultAllowed
        }
    }
}

impl ZclRequestPolicy for ViewGroup {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::SpecificRequired
    }
}

impl ZclRequestPolicy for RemoveAllGroups {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::DefaultAllowed
    }
}

impl ZclRequestPolicy for AddGroupIfIdentifying {
    fn default_response_policy(&self) -> DefaultResponsePolicy {
        DefaultResponsePolicy::DefaultAllowed
    }
}

#[cfg(test)]
mod tests {
    use zb_zcl::ota_upgrade::ImageId;

    use super::*;

    const IMAGE: ImageId = ImageId::new(0x1234, 0x5678, 0x90ab_cdef);

    #[test]
    fn upgrade_end_policy_depends_on_request_status() {
        assert_eq!(
            UpgradeEndRequest::new(UpgradeEndStatus::Success, IMAGE).default_response_policy(),
            DefaultResponsePolicy::SpecificRequired
        );
        for status in [
            UpgradeEndStatus::Abort,
            UpgradeEndStatus::InvalidImage,
            UpgradeEndStatus::RequireMoreImage,
        ] {
            assert_eq!(
                UpgradeEndRequest::new(status, IMAGE).default_response_policy(),
                DefaultResponsePolicy::DefaultAllowed
            );
        }
    }

    #[test]
    fn group_commands_have_different_response_requirements() {
        assert_eq!(
            GetGroupMembership::default().default_response_policy(),
            DefaultResponsePolicy::SpecificRequired
        );
        assert_eq!(
            RemoveAllGroups.default_response_policy(),
            DefaultResponsePolicy::DefaultAllowed
        );
    }
}
