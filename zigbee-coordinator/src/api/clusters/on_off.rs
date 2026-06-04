use zcl::Cluster;
use zcl::general::on_off;
use zcl::general::on_off::{Off, On, Toggle};
use zigbee::Endpoint;
use zigbee_hw::{Error, Metadata};

use crate::Coordinator;
use crate::transmitter::{Handle, Payload};

/// Trait for On/Off cluster operations.
pub trait OnOff {
    /// Turns the device on.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn on(
        &self,
        short_id: u16,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Turns the device off.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn off(
        &self,
        short_id: u16,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Toggle the device state.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn toggle(
        &self,
        short_id: u16,
        endpoint: Endpoint,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl OnOff for Coordinator {
    async fn on(&self, short_id: u16, endpoint: Endpoint) -> Result<(), Error> {
        let metadata = Metadata::for_cluster::<On>(None, None);
        let payload = Payload::Zcl {
            manufacturer_code: None,
            payload: Cluster::OnOff(on_off::Command::On(On)),
        };
        self.transmitter
            .unicast(short_id, endpoint, metadata, payload)
            .await
    }

    async fn off(&self, short_id: u16, endpoint: Endpoint) -> Result<(), Error> {
        let metadata = Metadata::for_cluster::<Off>(None, None);
        let payload = Payload::Zcl {
            manufacturer_code: None,
            payload: Cluster::OnOff(on_off::Command::Off(Off)),
        };
        self.transmitter
            .unicast(short_id, endpoint, metadata, payload)
            .await
    }

    async fn toggle(&self, short_id: u16, endpoint: Endpoint) -> Result<(), Error> {
        let metadata = Metadata::for_cluster::<Toggle>(None, None);
        let payload = Payload::Zcl {
            manufacturer_code: None,
            payload: Cluster::OnOff(on_off::Command::Toggle(Toggle)),
        };
        self.transmitter
            .unicast(short_id, endpoint, metadata, payload)
            .await
    }
}
