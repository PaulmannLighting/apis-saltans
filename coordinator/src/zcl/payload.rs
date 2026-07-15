//! General-purpose APS frame.

use bytes::Bytes;
use le_stream::ToLeStream;
use zb_core::{ClusterSpecific, Direction, Profiled};
use zb_zcl::{Command, Directed, Scope, Scoped};

/// A simplified APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Payload {
    aps_metadata: zb_hw::Metadata,
    zcl_metadata: Metadata,
    bytes: Bytes,
}

impl Payload {
    pub const fn new(
        aps_metadata: zb_hw::Metadata,
        zcl_metadata: Metadata,
        payload: Bytes,
    ) -> Self {
        Self {
            aps_metadata,
            zcl_metadata,
            bytes: payload,
        }
    }

    pub fn into_parts(self) -> (zb_hw::Metadata, Metadata, Bytes) {
        (self.aps_metadata, self.zcl_metadata, self.bytes)
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
    T: ClusterSpecific + Command + Directed + Profiled + ToLeStream,
{
    fn from(payload: T) -> Self {
        Self {
            aps_metadata: zb_hw::Metadata::new(T::PROFILE, <T as ClusterSpecific>::ID),
            zcl_metadata: Metadata {
                scope: T::SCOPE,
                direction: T::DIRECTION,
                disable_default_response: T::DISABLE_DEFAULT_RESPONSE,
                manufacturer_code: T::MANUFACTURER_CODE,
                command_id: <T as Command>::ID,
            },
            bytes: payload.to_le_stream().collect(),
        }
    }
}
