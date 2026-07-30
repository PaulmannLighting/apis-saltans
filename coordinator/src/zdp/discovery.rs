//! Helpers for serving local ZDP discovery requests.

use zb_core::node::{Descriptor, ServerMask};
use zb_core::short_id::{Device, ShortId};
use zb_core::{ByteSizedVec, Endpoint};
use zb_zdp::{SimpleDescriptor, Status};

/// Coordinator network address.
pub(super) const LOCAL_NWK_ADDRESS: u16 = ShortId::Coordinator.as_u16();

/// The node targeted by a descriptor discovery request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DescriptorTarget {
    /// The request targets the coordinator.
    Local,

    /// The request targets another allocated device address.
    Remote(Device),

    /// The request contains a reserved or broadcast address.
    Invalid,
}

/// Classify a descriptor request's network address of interest.
pub(super) fn descriptor_target(nwk_addr_of_interest: u16) -> DescriptorTarget {
    match ShortId::try_from(nwk_addr_of_interest) {
        Ok(ShortId::Coordinator) => DescriptorTarget::Local,
        Ok(ShortId::Device(device)) => DescriptorTarget::Remote(device),
        Ok(ShortId::Broadcast(_)) | Err(_) => DescriptorTarget::Invalid,
    }
}

/// Collect the unique application endpoints advertised by local descriptors.
pub(super) fn active_endpoints(
    descriptors: &[SimpleDescriptor],
) -> Result<ByteSizedVec<Endpoint>, Status> {
    let mut endpoints = ByteSizedVec::new();

    for descriptor in descriptors {
        let endpoint = descriptor.endpoint();

        if !matches!(endpoint, Endpoint::Application(_)) || endpoints.contains(&endpoint) {
            continue;
        }
        if endpoints.push(endpoint).is_err() {
            return Err(Status::InsufficientSpace);
        }
    }

    Ok(endpoints)
}

/// Find the local simple descriptor for an application endpoint.
pub(super) fn simple_descriptor(
    endpoint: Endpoint,
    descriptors: &[SimpleDescriptor],
) -> Result<SimpleDescriptor, Status> {
    if !matches!(endpoint, Endpoint::Application(_)) {
        return Err(Status::InvalidEndpoint);
    }

    descriptors
        .iter()
        .find(|descriptor| descriptor.endpoint() == endpoint)
        .cloned()
        .ok_or(Status::InvalidEndpoint)
}

/// Return the local system-server capabilities matching a discovery request.
pub(super) fn matching_server_mask(
    requested: ServerMask,
    descriptor: &Descriptor,
) -> Option<ServerMask> {
    let matched = requested & *descriptor.server_mask();
    (!matched.is_empty()).then_some(matched)
}

#[cfg(test)]
mod tests {
    use zb_core::node::{Descriptor, Flags, MacCapabilityFlags, ServerMask};
    use zb_core::short_id::Broadcast;
    use zb_core::{Endpoint, Profile};
    use zb_zdp::{AppFlags, Clusters, SimpleDescriptor, Status};

    use super::{
        DescriptorTarget, LOCAL_NWK_ADDRESS, active_endpoints, descriptor_target,
        matching_server_mask, simple_descriptor,
    };

    const FIRST_ENDPOINT: u8 = 1;
    const MANUFACTURER_CODE: u16 = 0;
    const MAXIMUM_BUFFER_SIZE: u8 = 82;
    const MAXIMUM_TRANSFER_SIZE: u16 = 82;
    const REMOTE_ADDRESS: u16 = 0x1234;
    const SECOND_ENDPOINT: u8 = 2;

    #[test]
    fn classifies_descriptor_targets() {
        assert_eq!(
            descriptor_target(LOCAL_NWK_ADDRESS),
            DescriptorTarget::Local
        );
        assert!(matches!(
            descriptor_target(REMOTE_ADDRESS),
            DescriptorTarget::Remote(device) if device.as_u16() == REMOTE_ADDRESS
        ));
        assert_eq!(
            descriptor_target(Broadcast::AllDevices.as_u16()),
            DescriptorTarget::Invalid
        );
    }

    #[test]
    fn collects_unique_application_endpoints() {
        let first = descriptor(FIRST_ENDPOINT);
        let second = descriptor(SECOND_ENDPOINT);
        let descriptors = [first.clone(), second, first];

        assert_eq!(
            active_endpoints(&descriptors).as_deref(),
            Ok(&[
                Endpoint::from(FIRST_ENDPOINT),
                Endpoint::from(SECOND_ENDPOINT),
            ][..])
        );
    }

    #[test]
    fn finds_only_advertised_application_descriptors() {
        let descriptor = descriptor(FIRST_ENDPOINT);

        assert_eq!(
            simple_descriptor(
                Endpoint::from(FIRST_ENDPOINT),
                std::slice::from_ref(&descriptor),
            ),
            Ok(descriptor)
        );
        assert_eq!(
            simple_descriptor(Endpoint::from(SECOND_ENDPOINT), &[]),
            Err(Status::InvalidEndpoint)
        );
        assert_eq!(
            simple_descriptor(Endpoint::Data, &[]),
            Err(Status::InvalidEndpoint)
        );
    }

    #[test]
    fn returns_only_matching_system_server_capabilities() {
        let requested = ServerMask::PRIMARY_TRUST_CENTER | ServerMask::NETWORK_MANAGER;
        let descriptor = node_descriptor(ServerMask::PRIMARY_TRUST_CENTER);

        assert_eq!(
            matching_server_mask(requested, &descriptor),
            Some(ServerMask::PRIMARY_TRUST_CENTER)
        );
        assert_eq!(
            matching_server_mask(ServerMask::NETWORK_MANAGER, &descriptor),
            None
        );
    }

    fn descriptor(endpoint: u8) -> SimpleDescriptor {
        SimpleDescriptor::new(
            Endpoint::from(endpoint),
            Profile::ZigbeeHomeAutomation,
            MANUFACTURER_CODE,
            AppFlags::empty(),
            Clusters::new(),
            Clusters::new(),
        )
    }

    fn node_descriptor(server_mask: ServerMask) -> Descriptor {
        Descriptor::new(
            Flags::default(),
            MacCapabilityFlags::default(),
            MANUFACTURER_CODE,
            MAXIMUM_BUFFER_SIZE,
            MAXIMUM_TRANSFER_SIZE,
            server_mask,
            MAXIMUM_TRANSFER_SIZE,
        )
    }
}
