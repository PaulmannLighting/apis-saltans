use macaddr::MacAddr8;
use tokio::sync::mpsc::Sender;
use zcl::global::read_attributes::{Command, Response};
use zcl::{ParseAttributeError, ReadableAttribute};
use zigbee::Application;
use zigbee_hw::Metadata;

use crate::transceiver::zcl::{Handle, Message, Payload};
use crate::{Coordinator, Error, NetworkManager};

/// Result of reading an attribute.
pub type ReadAttributeResult<T> =
    Result<<T as ReadableAttribute>::Attribute, ParseAttributeError<T>>;

/// Trait for reading attributes.
pub trait ReadAttributes {
    /// Read raw attributes from a device.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if the communication fails or if the response is not a valid [`Response`].
    fn read_attributes_raw(
        &self,
        ieee_address: MacAddr8,
        endpoint: Application,
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
        ieee_address: MacAddr8,
        endpoint: Application,
        attributes: Box<[T]>,
    ) -> impl Future<Output = Result<Box<[ReadAttributeResult<T>]>, Error>> + Send
    where
        Self: Sync,
        T: ReadableAttribute,
    {
        let ids = attributes.into_iter().map(Into::into).collect();

        async move {
            self.read_attributes_raw(ieee_address, endpoint, T::ID, T::MANUFACTURER_CODE, ids)
                .await
                .map(Into::into)
        }
    }
}

impl ReadAttributes for Coordinator {
    async fn read_attributes_raw(
        &self,
        ieee_address: MacAddr8,
        endpoint: Application,
        cluster: u16,
        manufacturer_code: Option<u16>,
        ids: Box<[u16]>,
    ) -> Result<Response, Error> {
        self.zcl
            .read_attributes_raw(
                self.network_manager
                    .get_short_id_from_ieee_address(ieee_address)
                    .await?
                    .ok_or(Error::UnknownDevice(ieee_address))?,
                endpoint,
                cluster,
                manufacturer_code,
                ids,
            )
            .await
    }
}

/// Internal trait for reading attributes using the short ID.
pub trait ReadAttributesInternal {
    /// Read raw attributes from a device.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if the communication fails or if the response is not a valid [`Response`].
    fn read_attributes_raw(
        &self,
        short_id: u16,
        endpoint: Application,
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
        short_id: u16,
        endpoint: Application,
        attributes: Box<[T]>,
    ) -> impl Future<Output = Result<Box<[ReadAttributeResult<T>]>, Error>> + Send
    where
        Self: Sync,
        T: ReadableAttribute,
    {
        let ids = attributes.into_iter().map(Into::into).collect();

        async move {
            self.read_attributes_raw(short_id, endpoint, T::ID, T::MANUFACTURER_CODE, ids)
                .await
                .map(Into::into)
        }
    }
}

impl ReadAttributesInternal for Sender<Message> {
    async fn read_attributes_raw(
        &self,
        short_id: u16,
        endpoint: Application,
        cluster: u16,
        manufacturer_code: Option<u16>,
        ids: Box<[u16]>,
    ) -> Result<Response, Error> {
        #[expect(unsafe_code)]
        // SAFETY: We construct matching metadata from the given cluster ID.
        // Since reading attributes is a global command, we don't need to validate the cluster ID.
        // Hence, the resulting metadata and command are guaranteed to match.
        let payload = unsafe {
            Payload::new(
                Metadata::new(cluster, None, None),
                manufacturer_code,
                Command::new(ids),
            )
        };

        self.communicate(short_id, endpoint, payload).await
    }
}
