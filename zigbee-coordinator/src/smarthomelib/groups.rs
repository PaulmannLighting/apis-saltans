use std::time::Duration;

use smarthomelib::{Rgb, ZigbeeGroups};

use crate::{Coordinator, Error};

impl ZigbeeGroups for Coordinator {
    async fn sync_group_membership(
        &self,
        _group_id: Self::GroupId,
        _endpoints: Vec<(Self::DeviceId, Self::EndpointId)>,
    ) -> Result<(), Self::Error> {
        Err(Error::Unsupported("zigbee groups"))
    }

    async fn group_on(&self, _group_id: Self::GroupId) -> Result<(), Self::Error> {
        Err(Error::Unsupported("zigbee groups"))
    }

    async fn group_off(&self, _group_id: Self::GroupId) -> Result<(), Self::Error> {
        Err(Error::Unsupported("zigbee groups"))
    }

    async fn group_toggle(&self, _group_id: Self::GroupId) -> Result<(), Self::Error> {
        Err(Error::Unsupported("zigbee groups"))
    }

    async fn group_move_to_level(
        &self,
        _group_id: Self::GroupId,
        _level: u8,
        _transition: Option<Duration>,
    ) -> Result<(), Self::Error> {
        Err(Error::Unsupported("zigbee groups"))
    }

    async fn group_move_to_color(
        &self,
        _group_id: Self::GroupId,
        _color: Rgb,
        _transition: Option<Duration>,
    ) -> Result<(), Self::Error> {
        Err(Error::Unsupported("zigbee groups"))
    }
}
