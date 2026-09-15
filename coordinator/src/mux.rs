use bytes::Bytes;
use log::{trace, warn};
use tokio::spawn;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::{Receiver, Sender};
use zb_aps::apsde::DataIndication;
use zb_hw::{
    ApsdeEvent as HardwareApsdeEvent, DeviceEvent as HardwareDeviceEvent, Event as HardwareEvent,
    NetworkEvent as HardwareNetworkEvent,
};

use self::aps_payload::ApsPayload;
use crate::event::EventSink;
use crate::{Device, Event, Network, NetworkError, aps, ota, zcl, zdp};

mod aps_payload;

/// Event multiplexer.
#[derive(Debug)]
pub struct Mux {
    events: EventSink,
    aps: aps::Aps,
    ota: Sender<ota::Message>,
    zcl: Sender<zcl::Message>,
    zdp: Sender<zdp::Message>,
}

impl Mux {
    /// Create a new multiplexer.
    pub const fn new(
        events: EventSink,
        aps: aps::Aps,
        ota: Sender<ota::Message>,
        zcl: Sender<zcl::Message>,
        zdp: Sender<zdp::Message>,
    ) -> Self {
        Self {
            events,
            aps,
            ota,
            zcl,
            zdp,
        }
    }

    /// Start the multiplexer.
    pub fn spawn<T, K>(
        hw_events: Receiver<HardwareEvent<T, K>>,
        events: EventSink,
        aps: aps::Aps,
        ota_tx: Sender<ota::Message>,
        zcl_tx: Sender<zcl::Message>,
        zdp_tx: Sender<zdp::Message>,
    ) where
        T: Send + 'static,
        K: Send + 'static,
    {
        spawn(Self::new(events, aps, ota_tx, zcl_tx, zdp_tx).run(hw_events));
    }

    /// Run the multiplexer.
    pub async fn run<T, K>(self, mut messages: Receiver<HardwareEvent<T, K>>)
    where
        T: Send,
        K: Send,
    {
        while let Some(event) = messages.recv().await {
            self.multiplex(event).await;
        }
        self.hardware_event_stream_closed().await;
    }

    async fn hardware_event_stream_closed(&self) {
        warn!("Hardware event stream closed; stopping coordinator protocol actors");
        self.events.emit(Event::Network(Network::Error(
            NetworkError::HardwareEventStreamClosed,
        )));
        let aps = self.aps.clone();
        let aps_shutdown = spawn(async move {
            aps.hardware_unavailable().await.unwrap_or_else(|error| {
                trace!("Failed to stop APS actor after hardware stream closure: {error}");
            });
        });
        let zcl = self.zcl.clone();
        let zcl_shutdown = spawn(async move {
            zcl.send(zcl::Message::HardwareUnavailable)
                .await
                .unwrap_or_else(|error| {
                    trace!("Failed to stop ZCL actor after hardware stream closure: {error}");
                });
        });
        let zdp = self.zdp.clone();
        let zdp_shutdown = spawn(async move {
            zdp.send(zdp::Message::HardwareUnavailable)
                .await
                .unwrap_or_else(|error| {
                    trace!("Failed to stop ZDP actor after hardware stream closure: {error}");
                });
        });
        let ota = self.ota.clone();
        let ota_shutdown = spawn(async move {
            ota.send(ota::Message::HardwareUnavailable)
                .await
                .unwrap_or_else(|error| {
                    trace!("Failed to stop OTA actor after hardware stream closure: {error}");
                });
        });
        for shutdown in [aps_shutdown, zcl_shutdown, zdp_shutdown, ota_shutdown] {
            shutdown.await.unwrap_or_else(|error| {
                trace!("Terminal notification task failed: {error}");
            });
        }
    }

    async fn multiplex<T, K>(&self, event: HardwareEvent<T, K>) {
        match event {
            HardwareEvent::Network(event) => self.multiplex_network_event(event).await,
            HardwareEvent::Device(event) => self.multiplex_device_event(&event),
            HardwareEvent::Apsde(event) => self.multiplex_apsde_event(event).await,
            _ => trace!("Ignoring unsupported hardware event"),
        }
    }

    async fn multiplex_network_event(&self, event: HardwareNetworkEvent) {
        match event {
            HardwareNetworkEvent::Up => {
                trace!("Network is up");
                self.events.emit(Event::Network(Network::Up));
            }
            HardwareNetworkEvent::Down => {
                trace!("Network is down");
                self.aps.network_down().await.unwrap_or_else(|error| {
                    trace!("Failed to notify APS actor that the network is down: {error}");
                });
                self.zcl
                    .send(zcl::Message::NetworkDown)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to notify ZCL actor that the network is down: {error}");
                    });
                self.zdp
                    .send(zdp::Message::NetworkDown)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to notify ZDP actor that the network is down: {error}");
                    });
                self.events.emit(Event::Network(Network::Down));
            }
            HardwareNetworkEvent::Opened => {
                trace!("Network has been opened");
                self.events.emit(Event::Network(Network::Opened));
            }
            HardwareNetworkEvent::Closed => {
                trace!("Network has been closed");
                self.events.emit(Event::Network(Network::Closed));
            }
            HardwareNetworkEvent::RouteError(error) => {
                trace!("Route error: {error}");
                self.events
                    .emit(Event::Network(Network::Error(NetworkError::Route(error))));
            }
            _ => trace!("Ignoring unsupported hardware network event"),
        }
    }

    fn multiplex_device_event(&self, event: &HardwareDeviceEvent) {
        match event {
            HardwareDeviceEvent::Joined(address) => {
                trace!("Device joined: {address}");
                self.events.emit(Event::Device(Device::Joined(*address)));
            }
            HardwareDeviceEvent::Rejoined { address, secured } => {
                trace!("Device joined: {address} (secured: {secured})");
                self.events.emit(Event::Device(Device::Rejoined {
                    address: *address,
                    secured: *secured,
                }));
            }
            HardwareDeviceEvent::Left(address) => {
                trace!("Device left: {address}");
                self.events.emit(Event::Device(Device::Left(*address)));
            }
            _ => trace!("Ignoring unsupported hardware device event"),
        }
    }

    async fn multiplex_apsde_event<T, K>(&self, event: HardwareApsdeEvent<T, K>) {
        match event {
            HardwareApsdeEvent::DataIndication {
                indication,
                zdo_response_required,
            } => {
                self.handle_data_indication(indication, zdo_response_required)
                    .await;
            }
            HardwareApsdeEvent::DataConfirm {
                counter,
                confirmation,
            } => {
                let status = confirmation.status();
                trace!(
                    "APS data confirmation for counter {counter}, destination {:?}: {status}",
                    confirmation.destination()
                );
                self.aps
                    .confirm(counter, status)
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to forward APS data confirmation: {error}");
                    });
            }
            _ => trace!("Ignoring unsupported hardware APS event"),
        }
    }

    async fn handle_data_indication<T, K>(
        &self,
        indication: DataIndication<Bytes, T, K>,
        zdo_response_required: bool,
    ) {
        let indication = indication.map_context(drop, drop);
        if !indication.metadata().status().is_success() {
            warn!(
                "Discarding unsuccessful APS data indication: {:?}",
                indication.metadata().status()
            );
            return;
        }

        let (metadata, asdu) = indication.into_parts();
        match ApsPayload::parse(&metadata, asdu) {
            Ok(payload) => {
                self.forward_received_message(
                    DataIndication::new(metadata, payload),
                    zdo_response_required,
                )
                .await;
            }
            Err(error) => warn!("Failed to parse APS data indication: {error}"),
        }
    }

    async fn forward_received_message(
        &self,
        indication: DataIndication<ApsPayload, (), ()>,
        zdo_response_required: bool,
    ) {
        let (metadata, payload) = indication.into_parts();

        match payload {
            ApsPayload::Zcl(frame) => {
                let indication = DataIndication::new(metadata, frame);

                self.zcl
                    .send(zcl::Message::Received { indication })
                    .await
                    .unwrap_or_else(|error| {
                        trace!("Failed to send ZCL message: {error}");
                    });
            }
            ApsPayload::Zdp(frame) => {
                let indication = DataIndication::new(metadata, frame);

                match self.zdp.try_send(zdp::Message::Received {
                    indication,
                    response_required: zdo_response_required,
                }) {
                    Ok(()) => {}
                    Err(TrySendError::Full(_)) => {
                        warn!("Discarding received ZDP frame because the actor inbox is full");
                    }
                    Err(TrySendError::Closed(_)) => {
                        trace!("Failed to send ZDP message: actor unavailable");
                    }
                }
            }
            ApsPayload::KeepAlive => {
                let source = metadata.source();
                let Some(source_address) = source.network_address() else {
                    warn!("Keep-Alive packet from non-network source: {source:?}");
                    return;
                };
                let Some(source_endpoint) = source.endpoint() else {
                    warn!("Keep-Alive packet from source without endpoint: {source:?}");
                    return;
                };
                let Ok(device_id) = source_address.as_u16().try_into().inspect_err(|id| {
                    warn!("Keep-Alive packet from invalid device id: {id:#06X}");
                }) else {
                    return;
                };

                self.events.emit(Event::Device(Device::KeepAlive(
                    crate::event::KeepAlive::new(device_id, source_endpoint),
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests;
