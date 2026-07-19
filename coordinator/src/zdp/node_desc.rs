//! Node Descriptor request processing helpers.

use zb_core::node::LogicalType;
use zb_core::short_id::ShortId;
use zb_zdp::Status;

/// Processing selected for an incoming Node Descriptor request.
pub(super) enum Action {
    /// Return the descriptor advertised by the local device.
    RespondWithLocalDescriptor,

    /// Resolve a nonlocal address against the NCP's known children.
    ResolveChild(u16),

    /// Send a response containing the specified error status.
    RespondWithError(Status),
}

/// Select Node Descriptor processing from the local device type and requested address.
pub(super) fn action(logical_type: LogicalType, nwk_addr_of_interest: u16) -> Action {
    if nwk_addr_of_interest == ShortId::Coordinator.as_u16() {
        return Action::RespondWithLocalDescriptor;
    }

    if logical_type == LogicalType::EndDevice {
        return Action::RespondWithError(Status::InvalidRequestType);
    }

    Action::ResolveChild(nwk_addr_of_interest)
}

/// Return the status for a child whose node descriptor is unavailable.
pub(super) const fn unavailable_child_status(child_is_known: bool) -> Status {
    if child_is_known {
        Status::NoDescriptor
    } else {
        Status::DeviceNotFound
    }
}

#[cfg(test)]
mod tests {
    use zb_core::node::LogicalType;
    use zb_zdp::Status;

    use super::{Action, action, unavailable_child_status};

    const LOCAL_NWK_ADDRESS: u16 = 0x0000;
    const REMOTE_NWK_ADDRESS: u16 = 0x1234;

    #[test]
    fn selects_local_descriptor_for_the_coordinator_address() {
        assert!(matches!(
            action(LogicalType::Coordinator, LOCAL_NWK_ADDRESS),
            Action::RespondWithLocalDescriptor
        ));
    }

    #[test]
    fn rejects_a_nonlocal_request_received_by_an_end_device() {
        assert!(matches!(
            action(LogicalType::EndDevice, REMOTE_NWK_ADDRESS),
            Action::RespondWithError(Status::InvalidRequestType)
        ));
    }

    #[test]
    fn resolves_a_nonlocal_request_received_by_a_router_or_coordinator() {
        assert!(matches!(
            action(LogicalType::Coordinator, REMOTE_NWK_ADDRESS),
            Action::ResolveChild(REMOTE_NWK_ADDRESS)
        ));
        assert!(matches!(
            action(LogicalType::Router, REMOTE_NWK_ADDRESS),
            Action::ResolveChild(REMOTE_NWK_ADDRESS)
        ));
    }

    #[test]
    fn reports_the_status_for_an_unavailable_child_descriptor() {
        assert_eq!(unavailable_child_status(false), Status::DeviceNotFound);
        assert_eq!(unavailable_child_status(true), Status::NoDescriptor);
    }
}
