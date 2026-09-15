//! Common wire payload for Network Address and IEEE Address responses.

use le_stream::{FromLeStream, ToLeStream};
use zb_core::{ByteSizedVec, IeeeAddress};

use crate::Status;

const NO_ASSOCIATED_DEVICES: u8 = 0;

/// Address-response fields shared by the two discovery commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(super) struct AddressResponse {
    status: u8,
    ieee_addr_remote_dev: Option<IeeeAddress>,
    nwk_addr_remote_dev: Option<u16>,
    num_assoc_dev: Option<u8>,
    start_index: Option<u8>,
    nwk_addr_assoc_dev_list: Box<[u16]>,
}

impl AddressResponse {
    /// Create a successful response for one device.
    pub(super) fn single(ieee_addr_remote_dev: IeeeAddress, nwk_addr_remote_dev: u16) -> Self {
        Self {
            status: Status::Success.into(),
            ieee_addr_remote_dev: Some(ieee_addr_remote_dev),
            nwk_addr_remote_dev: Some(nwk_addr_remote_dev),
            num_assoc_dev: None,
            start_index: None,
            nwk_addr_assoc_dev_list: Box::default(),
        }
    }

    /// Create a successful response containing an associated-device page.
    pub(super) fn extended(
        ieee_addr_remote_dev: IeeeAddress,
        nwk_addr_remote_dev: u16,
        start_index: u8,
        nwk_addr_assoc_dev_list: ByteSizedVec<u16>,
    ) -> Self {
        Self {
            num_assoc_dev: Some(u8::try_from(nwk_addr_assoc_dev_list.len()).unwrap_or(u8::MAX)),
            start_index: if nwk_addr_assoc_dev_list.is_empty() {
                None
            } else {
                Some(start_index)
            },
            nwk_addr_assoc_dev_list: nwk_addr_assoc_dev_list
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            ..Self::single(ieee_addr_remote_dev, nwk_addr_remote_dev)
        }
    }

    /// Create a status-only response.
    pub(super) fn failure(status: Status) -> Self {
        Self {
            status: status.into(),
            ieee_addr_remote_dev: None,
            nwk_addr_remote_dev: None,
            num_assoc_dev: None,
            start_index: None,
            nwk_addr_assoc_dev_list: Box::default(),
        }
    }

    /// Decode the status, preserving unknown values as errors.
    pub(super) fn status(&self) -> Result<Status, u8> {
        self.status.try_into()
    }
}

impl FromLeStream for AddressResponse {
    fn from_le_stream<T>(mut bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let status = <u8 as FromLeStream>::from_le_stream(&mut bytes)?;
        let ieee_addr_remote_dev =
            <Option<IeeeAddress> as FromLeStream>::from_le_stream(&mut bytes)?;
        let nwk_addr_remote_dev = if ieee_addr_remote_dev.is_some() {
            Some(<u16 as FromLeStream>::from_le_stream(&mut bytes)?)
        } else {
            None
        };
        let num_assoc_dev = <Option<u8> as FromLeStream>::from_le_stream(&mut bytes)?;
        let start_index =
            if num_assoc_dev.is_some_and(|num_assoc_dev| num_assoc_dev > NO_ASSOCIATED_DEVICES) {
                Some(<u8 as FromLeStream>::from_le_stream(&mut bytes)?)
            } else {
                None
            };
        let mut nwk_addr_assoc_dev_list =
            Vec::with_capacity(usize::from(num_assoc_dev.unwrap_or_default()));

        for _ in NO_ASSOCIATED_DEVICES..num_assoc_dev.unwrap_or_default() {
            nwk_addr_assoc_dev_list.push(<u16 as FromLeStream>::from_le_stream(&mut bytes)?);
        }

        Some(Self {
            status,
            ieee_addr_remote_dev,
            nwk_addr_remote_dev,
            num_assoc_dev,
            start_index,
            nwk_addr_assoc_dev_list: nwk_addr_assoc_dev_list.into_boxed_slice(),
        })
    }
}

impl ToLeStream for AddressResponse {
    type Iter = std::vec::IntoIter<u8>;

    fn to_le_stream(self) -> Self::Iter {
        let mut bytes = Vec::new();

        bytes.extend(<u8 as ToLeStream>::to_le_stream(self.status));
        bytes.extend(<Option<IeeeAddress> as ToLeStream>::to_le_stream(
            self.ieee_addr_remote_dev,
        ));
        bytes.extend(<Option<u16> as ToLeStream>::to_le_stream(
            self.nwk_addr_remote_dev,
        ));

        if let Some(num_assoc_dev) = self.num_assoc_dev {
            bytes.extend(<u8 as ToLeStream>::to_le_stream(num_assoc_dev));

            if num_assoc_dev > NO_ASSOCIATED_DEVICES {
                bytes.extend(<u8 as ToLeStream>::to_le_stream(
                    self.start_index.unwrap_or_default(),
                ));

                for nwk_addr_assoc_dev in self.nwk_addr_assoc_dev_list {
                    bytes.extend(<u16 as ToLeStream>::to_le_stream(nwk_addr_assoc_dev));
                }
            }
        }

        bytes.into_iter()
    }
}
