//! Smarthome-facing Zigbee runtime.

mod zigbee_api;

use self::zigbee_api::ZigbeeApiHandles;

/// Zigbee protocol integration runtime.
#[derive(Debug)]
pub struct ZigbeeRuntime;

impl ZigbeeRuntime {
    /// Start the Zigbee runtime actors and return its API handle.
    #[must_use]
    pub fn start() -> ::smarthomelib::ZigbeeApiHandle {
        let handles = ZigbeeApiHandles::noop();

        // Future shape once `zigbee-coordinator` exposes stable handle accessors:
        // let coordinator = zigbee_coordinator::Coordinator::start(hardware).await?;
        // let handles = ZigbeeApiHandles {
        //     network_manager: coordinator.network_manager_handle(),
        //     zcl_transceiver: coordinator.zcl_transceiver_handle(),
        // };

        zigbee_api::start(handles)
    }
}
