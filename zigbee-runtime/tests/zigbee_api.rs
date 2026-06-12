//! Public API coverage for the no-op Zigbee runtime.

use std::time::Duration;

use log as _;
use smarthomelib::{
    DeviceId, EndpointCommand, EndpointId, ZigbeeEndpointNumber, ZigbeeIeeeAddress,
};
use tokio::sync::mpsc;
use zigbee_runtime::ZigbeeRuntime;

#[tokio::test]
async fn list_devices_when_runtime_has_no_devices_then_returns_empty_list() {
    let handle = ZigbeeRuntime::start();

    assert!(
        handle
            .list_devices()
            .await
            .expect("list devices")
            .is_empty()
    );
}

#[tokio::test]
async fn restore_devices_when_requested_then_acknowledges() {
    ZigbeeRuntime::start()
        .restore_devices(Vec::new())
        .await
        .expect("restore devices");
}

#[tokio::test]
async fn allow_joins_when_requested_then_acknowledges() {
    ZigbeeRuntime::start()
        .allow_joins(Duration::from_secs(1))
        .await
        .expect("allow joins");
}

#[tokio::test]
async fn execute_command_when_requested_then_acknowledges() {
    ZigbeeRuntime::start()
        .execute_command(zigbee_endpoint(), EndpointCommand::On)
        .await
        .expect("execute command");
}

#[tokio::test]
async fn request_device_update_when_requested_then_acknowledges() {
    ZigbeeRuntime::start()
        .request_device_update(zigbee_device())
        .await
        .expect("request device update");
}

#[tokio::test]
async fn subscribe_device_events_when_noop_runtime_then_closes_subscription_sender() {
    let handle = ZigbeeRuntime::start();
    let (device_events, mut device_event_receiver) = mpsc::channel(1);

    handle
        .subscribe_device_events(device_events)
        .await
        .expect("subscribe device events");

    assert_eq!(device_event_receiver.recv().await, None);
}

#[tokio::test]
async fn subscribe_interaction_events_when_noop_runtime_then_closes_subscription_sender() {
    let handle = ZigbeeRuntime::start();
    let (interaction_events, mut interaction_event_receiver) = mpsc::channel(1);

    handle
        .subscribe_interaction_events(interaction_events)
        .await
        .expect("subscribe interaction events");

    assert_eq!(interaction_event_receiver.recv().await, None);
}

const fn zigbee_device() -> DeviceId {
    DeviceId::Zigbee {
        ieee_address: ZigbeeIeeeAddress(0x1122_3344_5566_7788),
    }
}

const fn zigbee_endpoint() -> EndpointId {
    EndpointId::Zigbee {
        owning_device_id: ZigbeeIeeeAddress(0x1122_3344_5566_7788),
        endpoint_number: ZigbeeEndpointNumber(1),
    }
}
