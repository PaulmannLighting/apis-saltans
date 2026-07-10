//! General-purpose APS frame.

use apis_saltans_core::{ClusterSpecific, Direction, Profiled};
use apis_saltans_zcl::{Command, Scope, Scoped};
use bytes::Bytes;
use le_stream::ToLeStream;

/// A simplified APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Payload {
    aps_metadata: apis_saltans_hw::Metadata,
    zcl_metadata: Metadata,
    payload: Bytes,
}

impl Payload {
    pub fn new(
        aps_metadata: apis_saltans_hw::Metadata,
        zcl_metadata: Metadata,
        payload: Bytes,
    ) -> Self {
        Self {
            aps_metadata,
            zcl_metadata,
            payload,
        }
    }

    pub fn into_parts(self) -> (apis_saltans_hw::Metadata, Metadata, Bytes) {
        (self.aps_metadata, self.zcl_metadata, self.payload)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Metadata {
    pub(crate) scope: Scope,
    pub(crate) direction: Direction,
    pub(crate) disable_default_response: bool,
    pub(crate) manufacturer_code: Option<u16>,
    pub(crate) command_id: u8,
}

impl<T> From<T> for Payload
where
    T: ClusterSpecific + Command + Profiled + ToLeStream,
{
    fn from(payload: T) -> Self {
        Self {
            aps_metadata: apis_saltans_hw::Metadata::new(T::PROFILE, <T as ClusterSpecific>::ID),
            zcl_metadata: Metadata {
                scope: T::SCOPE,
                direction: T::DIRECTION,
                disable_default_response: T::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: T::MANUFACTURER_CODE,
                command_id: <T as Command>::ID,
            },
            payload: payload.to_le_stream().collect(),
        }
    }
}
