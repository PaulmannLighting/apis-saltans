use zcl::global::read_attributes;
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
    /// Returns an [Error] if the communication fails or if the response
    /// is not a valid [`read_attributes::Response`].
    fn read_attributes(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        ids: &[u16],
    ) -> impl Future<Output = Result<read_attributes::Response, Error>>;
}

impl ReadAttributes for Coordinator {
    async fn read_attributes(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        ids: &[u16],
    ) -> Result<read_attributes::Response, Error> {
        let cluster = self
            .zcl_transceiver
            .communicate(
                address,
                endpoint,
                Payload::new(
                    Metadata::new(cluster, None, None),
                    manufacturer_code,
                    Cluster::Global(global::Command::ReadAttributes(
                        read_attributes::Command::new(ids.into()),
                    )),
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
