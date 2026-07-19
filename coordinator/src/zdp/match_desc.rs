//! Match Descriptor request processing helpers.

use zb_core::ByteSizedVec;
use zb_core::node::LogicalType;
use zb_core::short_id::ShortId;
use zb_zdp::{MatchDescReq, SimpleDescriptor, Status};

const PROFILE_ID_WILDCARD: u16 = u16::MAX;

/// Processing selected for an incoming Match Descriptor request.
pub(super) enum Action {
    /// Match the request against descriptors advertised by the local NCP.
    MatchLocalDescriptors,

    /// Resolve a nonlocal address against the NCP's known devices.
    MatchRemoteDevice(u16),

    /// Send an error response with an empty match list.
    RespondWithError(Status),

    /// Suppress the response as required for an unsuitable broadcast request.
    Ignore,
}

/// Select Match Descriptor processing from the local device type and request addressing.
pub(super) fn action(
    logical_type: LogicalType,
    nwk_addr_of_interest: u16,
    request_was_broadcast: bool,
) -> Action {
    let is_local_address = nwk_addr_of_interest == ShortId::Coordinator.as_u16();
    let is_broadcast_address = matches!(
        ShortId::try_from(nwk_addr_of_interest),
        Ok(ShortId::Broadcast(_))
    );

    if is_local_address || is_broadcast_address {
        return Action::MatchLocalDescriptors;
    }

    if logical_type == LogicalType::EndDevice {
        return if request_was_broadcast {
            Action::Ignore
        } else {
            Action::RespondWithError(Status::InvalidRequestType)
        };
    }

    Action::MatchRemoteDevice(nwk_addr_of_interest)
}

/// Collect each matching endpoint once.
pub(super) fn matching_endpoints(
    match_desc_req: &MatchDescReq,
    descriptors: &[SimpleDescriptor],
) -> Option<ByteSizedVec<u8>> {
    let mut matches = ByteSizedVec::new();

    for descriptor in descriptors {
        let endpoint = descriptor.endpoint_id();

        if simple_descriptor_matches(match_desc_req, descriptor)
            && !matches.contains(&endpoint)
            && matches.push(endpoint).is_err()
        {
            return None;
        }
    }

    Some(matches)
}

/// Return whether a Simple Descriptor satisfies the Match Descriptor criteria.
fn simple_descriptor_matches(match_desc_req: &MatchDescReq, descriptor: &SimpleDescriptor) -> bool {
    let profile_matches = match_desc_req.profile_id() == PROFILE_ID_WILDCARD
        || descriptor.profile_id() == match_desc_req.profile_id();
    let input_cluster_matches = match_desc_req
        .in_cluster_list()
        .iter()
        .any(|cluster| descriptor.input_clusters().contains(cluster));
    let output_cluster_matches = match_desc_req
        .out_cluster_list()
        .iter()
        .any(|cluster| descriptor.output_clusters().contains(cluster));

    profile_matches && (input_cluster_matches || output_cluster_matches)
}

#[cfg(test)]
mod tests {
    use zb_core::node::LogicalType;
    use zb_core::short_id::Broadcast;
    use zb_core::{Endpoint, Profile};
    use zb_zdp::{AppFlags, MatchDescReq, SimpleDescriptor, Status};

    use super::{
        Action, PROFILE_ID_WILDCARD, action, matching_endpoints, simple_descriptor_matches,
    };

    const APPLICATION_ENDPOINT: u8 = 0x01;
    const DEVICE_ID: u16 = 0x0001;
    const LOCAL_NWK_ADDRESS: u16 = 0x0000;
    const REMOTE_NWK_ADDRESS: u16 = 0x1234;
    const INPUT_CLUSTER: u16 = 0x0006;
    const OUTPUT_CLUSTER: u16 = 0x0008;
    const NON_MATCHING_CLUSTER: u16 = 0x0005;

    fn descriptor(profile: Profile) -> SimpleDescriptor {
        SimpleDescriptor::new(
            Endpoint::try_from(APPLICATION_ENDPOINT).expect("application endpoint must be valid"),
            profile,
            DEVICE_ID,
            AppFlags::empty(),
            std::iter::once(INPUT_CLUSTER).collect(),
            std::iter::once(OUTPUT_CLUSTER).collect(),
        )
    }

    fn request(
        profile_id: u16,
        input_clusters: impl IntoIterator<Item = u16>,
        output_clusters: impl IntoIterator<Item = u16>,
    ) -> MatchDescReq {
        MatchDescReq::new(
            LOCAL_NWK_ADDRESS,
            profile_id,
            input_clusters.into_iter().collect(),
            output_clusters.into_iter().collect(),
        )
    }

    #[test]
    fn matches_when_any_requested_input_cluster_matches() {
        let request = request(
            Profile::ZigbeeHomeAutomation.into(),
            [NON_MATCHING_CLUSTER, INPUT_CLUSTER],
            [],
        );

        assert!(simple_descriptor_matches(
            &request,
            &descriptor(Profile::ZigbeeHomeAutomation)
        ));
    }

    #[test]
    fn matches_when_any_requested_output_cluster_matches() {
        let request = request(
            Profile::ZigbeeHomeAutomation.into(),
            [NON_MATCHING_CLUSTER],
            [OUTPUT_CLUSTER],
        );

        assert!(simple_descriptor_matches(
            &request,
            &descriptor(Profile::ZigbeeHomeAutomation)
        ));
    }

    #[test]
    fn wildcard_profile_matches_a_descriptor_profile() {
        let request = request(PROFILE_ID_WILDCARD, [INPUT_CLUSTER], []);

        assert!(simple_descriptor_matches(
            &request,
            &descriptor(Profile::ZigbeeHomeAutomation)
        ));
    }

    #[test]
    fn rejects_requests_without_a_matching_cluster_or_profile() {
        let descriptor = descriptor(Profile::ZigbeeHomeAutomation);
        let empty_request = request(Profile::ZigbeeHomeAutomation.into(), [], []);
        let wrong_profile_request =
            request(Profile::BuildingAutomation.into(), [INPUT_CLUSTER], []);

        assert!(!simple_descriptor_matches(&empty_request, &descriptor));
        assert!(!simple_descriptor_matches(
            &wrong_profile_request,
            &descriptor
        ));
    }

    #[test]
    fn adds_a_matching_endpoint_only_once() {
        let request = request(Profile::ZigbeeHomeAutomation.into(), [INPUT_CLUSTER], []);
        let descriptor = descriptor(Profile::ZigbeeHomeAutomation);
        let descriptors = [descriptor.clone(), descriptor];

        assert_eq!(
            matching_endpoints(&request, &descriptors).as_deref(),
            Some(&[APPLICATION_ENDPOINT][..])
        );
    }

    #[test]
    fn selects_actions_from_device_type_address_and_delivery_mode() {
        assert!(matches!(
            action(LogicalType::Coordinator, LOCAL_NWK_ADDRESS, false),
            Action::MatchLocalDescriptors
        ));
        assert!(matches!(
            action(LogicalType::Router, Broadcast::AllDevices.into(), false),
            Action::MatchLocalDescriptors
        ));
        assert!(matches!(
            action(LogicalType::EndDevice, REMOTE_NWK_ADDRESS, true),
            Action::Ignore
        ));
        assert!(matches!(
            action(LogicalType::EndDevice, REMOTE_NWK_ADDRESS, false),
            Action::RespondWithError(Status::InvalidRequestType)
        ));
        assert!(matches!(
            action(LogicalType::Router, REMOTE_NWK_ADDRESS, false),
            Action::MatchRemoteDevice(REMOTE_NWK_ADDRESS)
        ));
    }
}
