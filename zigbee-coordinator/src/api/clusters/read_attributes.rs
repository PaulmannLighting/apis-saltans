use zcl::global::read_attributes::{Command, Response};
use zcl::{ParseAttributeError, ReadableAttribute};
use zigbee::{Address, Endpoint};
use zigbee_hw::Metadata;

use crate::transceiver::zcl::{Handle, Payload};
use crate::{Coordinator, Error};

type ReadAttributeResult<T> = Result<<T as ReadableAttribute>::Attribute, ParseAttributeError<T>>;

/// Trait for reading attributes.
pub trait ReadAttributes {
    /// Read raw attributes from a device.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if the communication fails or if the response is not a valid [`Response`].
    fn read_attributes_raw(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        ids: Box<[u16]>,
    ) -> impl Future<Output = Result<Response, Error>> + Send;

    /// Read attributes from a device.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if the communication fails or if the response is not a valid [`Response`].
    fn read_attributes<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        attributes: Box<[T]>,
    ) -> impl Future<Output = Result<Box<[ReadAttributeResult<T>]>, Error>> + Send
    where
        Self: Sync,
        T: ReadableAttribute,
    {
        let ids = attributes.into_iter().map(Into::into).collect();

        async move {
            self.read_attributes_raw(address, endpoint, T::ID, T::MANUFACTURER_CODE, ids)
                .await
                .map(Into::into)
        }
    }
}

impl ReadAttributes for Coordinator {
    async fn read_attributes_raw(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        ids: Box<[u16]>,
    ) -> Result<Response, Error> {
        self.zcl_transceiver
            .communicate(
                address.short_id(),
                endpoint,
                Payload::new(
                    Metadata::new(cluster, None, None),
                    manufacturer_code,
                    Command::new(ids),
                ),
            )
            .await
    }
}
