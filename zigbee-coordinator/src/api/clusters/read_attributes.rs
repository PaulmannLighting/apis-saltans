use zcl::global::read_attributes::{Command, Response};
use zcl::{Cluster, global};
use zigbee::{Address, Endpoint};
use zigbee_hw::{Error, Metadata};

use crate::Coordinator;
use crate::transceiver::zcl::{Handle, Payload};

/// Trait for reading attributes.
pub trait ReadAttributes {
    /// Read attributes from a device.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if the communication fails or if the response is not a valid [`Response`].
    fn read_attributes(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        ids: &[u16],
    ) -> impl Future<Output = Result<Response, Error>>;

    /// Read native attributes from a device.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if the communication fails or if the response is not a valid [`Response`].
    fn read_attributes_native(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        ids: &[u16],
    ) -> impl Future<Output = Result<Response, Error>> {
        self.read_attributes(address, endpoint, cluster, None, ids)
    }

    /// Read attributes from a device for a specific manufacturer code.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if the communication fails or if the response is not a valid [`Response`].
    fn read_attributes_manufacturer(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: u16,
        ids: &[u16],
    ) -> impl Future<Output = Result<Response, Error>> {
        self.read_attributes(address, endpoint, cluster, Some(manufacturer_code), ids)
    }
}

impl ReadAttributes for Coordinator {
    async fn read_attributes(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        ids: &[u16],
    ) -> Result<Response, Error> {
        let cluster = self
            .zcl_transceiver
            .communicate(
                address,
                endpoint,
                Payload::new(
                    Metadata::new(cluster, None, None),
                    manufacturer_code,
                    Command::new(ids.into()).into(),
                ),
            )
            .await?;

        if let Cluster::Global(global::Command::ReadAttributesResponse(response)) = cluster {
            Ok(response)
        } else {
            todo!()
        }
    }
}
