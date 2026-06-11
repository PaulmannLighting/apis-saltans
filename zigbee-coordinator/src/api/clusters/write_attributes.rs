use std::collections::BTreeSet;

use either::{Either, Left, Right};
use zcl::WritableAttribute;
use zcl::global::write_attributes::{Command, Record, Response};
use zigbee::{Address, Endpoint};
use zigbee_hw::Metadata;

pub use self::error::Error as WriteAttributesError;
use crate::api::clusters::write_attributes::error::Evaluate;
use crate::transceiver::zcl::{Handle, Payload};
use crate::{Coordinator, Error};

mod error;

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

    /// Write raw attributes to a device.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the communication fails or if the response is not a valid [`Response`]
    /// or an [`WriteAttributesError`] if the writing of the attributes failed.
    fn write_attributes<T>(
        &self,
        address: Address,
        endpoint: Endpoint,
        records: Box<[T]>,
    ) -> impl Future<Output = Result<(), Either<Error, Vec<WriteAttributesError>>>> + Send
    where
        Self: Sync,
        T: WritableAttribute,
    {
        let mut ids = BTreeSet::new();
        let records = records
            .into_iter()
            .map(|record| {
                ids.insert(record.id());
                record.into()
            })
            .collect();

        async move {
            self.write_attributes_raw(address, endpoint, T::ID, T::MANUFACTURER_CODE, records)
                .await
                .map_err(Left)?
                .evaluate(ids)
                .map_err(Right)
        }
    }
}

impl WriteAttributes for Coordinator {
    async fn write_attributes_raw(
        &self,
        address: Address,
        endpoint: Endpoint,
        cluster: u16,
        manufacturer_code: Option<u16>,
        records: Box<[Record]>,
    ) -> Result<Response, Error> {
        self.zcl_transceiver
            .communicate(
                address,
                endpoint,
                Payload::new(
                    Metadata::new(cluster, None, None),
                    manufacturer_code,
                    Command::new(records),
                ),
            )
            .await
    }
}
