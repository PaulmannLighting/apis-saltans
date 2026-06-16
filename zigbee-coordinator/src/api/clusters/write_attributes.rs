use std::borrow::Borrow;

use tokio::sync::mpsc::Sender;
use zcl::WritableAttribute;
use zcl::global::write_attributes::{Command, Record, Response};
use zigbee::{Address, Endpoint};
use zigbee_hw::Metadata;

use crate::Error;
use crate::transceiver::zcl::{Handle, Message, Payload};

/// Trait to write attributes to a device.
pub trait WriteAttributes {
    /// Write raw attributes to a device.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication fails or if the response is not a valid [`Response`].
    fn write_attributes_raw(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        records: Box<[Record]>,
    ) -> impl Future<Output = Result<Response, Error>> + Send;

    /// Write attributes to a device.
    ///
    /// # Returns
    ///
    /// Returns a [`Vec`] of [`Result`]s, where each [`Result`] contains the status of the write operation for each attribute.
    ///
    /// - `Ok(id)`: The attribute was successfully written.
    /// - `Err(id)`: The attribute could not be written.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication fails or if the response is not a valid [`Response`].
    fn write_attributes<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        attributes: Box<[T]>,
    ) -> impl Future<Output = Result<Vec<Result<u16, u16>>, Error>> + Send
    where
        Self: Sync,
        T: WritableAttribute,
    {
        let records = attributes.into_iter().map(Into::into).collect();

        async move {
            self.write_attributes_raw(address, endpoint, T::ID, T::MANUFACTURER_CODE, records)
                .await
                .map(|response| response.into_iter().map(TryInto::try_into).collect())
        }
    }
}

impl<T> WriteAttributes for T
where
    T: Borrow<Sender<Message>> + Sync,
{
    async fn write_attributes_raw(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        records: Box<[Record]>,
    ) -> Result<Response, Error> {
        #[expect(unsafe_code)]
        // SAFETY: We construct matching metadata from the given cluster ID.
        // Since witing attributes is a global command, we don't need to validate the cluster ID.
        // Hence, the resulting metadata and command are guaranteed to match.
        let payload = unsafe {
            Payload::new(
                Metadata::new(cluster, None, None),
                manufacturer_code,
                Command::new(records),
            )
        };

        self.communicate(address.short_id(), endpoint, payload)
            .await
    }
}
