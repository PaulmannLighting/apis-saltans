use tokio::sync::mpsc::Sender;
use zb_core::destination::Device;
use zb_zcl::global::configure_reporting;
use zb_zcl::{ParseAttributeError, Readable, Reportable, Writable};

use self::configure_reporting_request::ConfigureReportingRequest;
use self::read_attributes_request::ReadAttributesRequest;
use self::write_attributes_request::WriteAttributesRequest;
use crate::transceiver::zcl::{Handle, Message};
use crate::{Coordinator, Error};

mod configure_reporting_request;
mod read_attributes_request;
mod write_attributes_request;

/// Result of reading an attribute.
pub type ReadAttributeResult<T> = Result<<T as Readable>::Attribute, ParseAttributeError<T>>;

/// Result of writing an attribute.
pub type WriteAttributeResult = Result<u16, u16>;

/// Trait for configuring, reading, and writing attributes.
pub trait Attributes {
    /// Configure a device to send reports for attributes.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if communication fails or the response is invalid.
    fn configure_reporting<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> impl Future<Output = Result<configure_reporting::Response, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Reportable + Send, IntoIter: Send> + Send;

    /// Read attributes from a device.
    ///
    /// # Errors
    ///
    /// Returns an [Error] if communication fails or the response is invalid.
    fn read<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> impl Future<Output = Result<Box<[ReadAttributeResult<T::Item>]>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Readable + Send, IntoIter: Send> + Send;

    /// Write attributes to a device.
    ///
    /// Each result contains the ID of an attribute that was written successfully or failed.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if communication fails or the response is invalid.
    fn write<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> impl Future<Output = Result<Vec<WriteAttributeResult>, Error>> + Send
    where
        Self: Sync,
        T: IntoIterator<Item: Writable + Send, IntoIter: Send> + Send;
}

impl Attributes for Sender<Message> {
    async fn configure_reporting<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<configure_reporting::Response, Error>
    where
        T: IntoIterator<Item: Reportable + Send, IntoIter: Send> + Send,
    {
        self.communicate(device, ConfigureReportingRequest::new(attributes))
            .await
    }

    async fn read<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<Box<[ReadAttributeResult<T::Item>]>, Error>
    where
        T: IntoIterator<Item: Readable + Send, IntoIter: Send> + Send,
    {
        let response = self
            .communicate(device, ReadAttributesRequest::new(attributes))
            .await?;

        Ok(response.into())
    }

    async fn write<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<Vec<WriteAttributeResult>, Error>
    where
        T: IntoIterator<Item: Writable + Send, IntoIter: Send> + Send,
    {
        let response = self
            .communicate(device, WriteAttributesRequest::new(attributes))
            .await?;

        Ok(response.into_iter().map(TryInto::try_into).collect())
    }
}

impl Attributes for Coordinator {
    async fn configure_reporting<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<configure_reporting::Response, Error>
    where
        T: IntoIterator<Item: Reportable + Send, IntoIter: Send> + Send,
    {
        self.zcl.configure_reporting(device, attributes).await
    }

    async fn read<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<Box<[ReadAttributeResult<T::Item>]>, Error>
    where
        T: IntoIterator<Item: Readable + Send, IntoIter: Send> + Send,
    {
        self.zcl.read(device, attributes).await
    }

    async fn write<T>(
        &self,
        device: Device,
        attributes: T,
    ) -> Result<Vec<WriteAttributeResult>, Error>
    where
        T: IntoIterator<Item: Writable + Send, IntoIter: Send> + Send,
    {
        self.zcl.write(device, attributes).await
    }
}
