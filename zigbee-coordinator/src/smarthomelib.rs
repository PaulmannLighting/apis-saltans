#![cfg(feature = "smarthomelib")]
//! SmartHomeLib integration.

use std::time::Duration;

use macaddr::MacAddr8;
use smarthomelib::protocol::{ColorControl, OnOff};
use smarthomelib::{Protocol, Rgb, Xy};
use zcl::Options;
use zigbee::{Endpoint, FromDeciSeconds, IntoDeciSeconds};

use crate::{Coordinator, Error};

impl Protocol for Coordinator {
    type Error = Error;
    type DeviceId = MacAddr8;
    type EndpointId = Endpoint;
}

impl OnOff for Coordinator {
    async fn on(
        &self,
        device: Self::DeviceId,
        endpoint: Self::EndpointId,
    ) -> Result<(), Self::Error> {
        crate::OnOff::on(self, device, endpoint).await
    }

    async fn off(
        &self,
        device: Self::DeviceId,
        endpoint: Self::EndpointId,
    ) -> Result<(), Self::Error> {
        crate::OnOff::off(self, device, endpoint).await
    }

    async fn toggle(
        &self,
        device: Self::DeviceId,
        endpoint: Self::EndpointId,
    ) -> Result<(), Self::Error> {
        crate::OnOff::toggle(self, device, endpoint).await
    }
}

impl ColorControl for Coordinator {
    async fn move_to_color(
        &self,
        device: Self::DeviceId,
        endpoint: Self::EndpointId,
        color: Rgb,
        transition_time: Duration,
    ) -> Result<Duration, Self::Error> {
        let xy: Xy = color.into();
        let deci_seconds: u16 = transition_time
            .into_deci_seconds()
            .try_into()
            .unwrap_or(u16::MAX);
        crate::ColorControl::move_to_xy(
            self,
            device,
            endpoint,
            xy.x(),
            xy.y(),
            deci_seconds,
            Options::default(),
        )
        .await?;
        Ok(Duration::from_deci_seconds(deci_seconds))
    }
}
