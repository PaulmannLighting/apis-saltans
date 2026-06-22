use log::{trace, warn};
use serde::{Deserialize, Serialize};
use zcl::general::basic::readable::{Attribute, Id};
use zcl::general::basic::{DateCode, PowerSource};

use crate::ReadAttributeResult;

/// The attributes we want to discover.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Attributes {
    /// The ZCL version of the device.
    pub zcl_version: Option<u8>,

    /// The application version of the device.
    pub application_version: Option<u8>,

    /// The stack version of the device.
    pub stack_version: Option<u8>,

    /// The hardware version of the device.
    pub hw_version: Option<u8>,

    /// The manufacturer name of the device.
    pub manufacturer_name: Option<String>,

    /// The model identifier of the device.
    pub model_identifier: Option<String>,

    /// The date code of the device.
    pub date_code: Option<DateCode>,

    /// The power source of the device.
    pub power_source: Option<PowerSource>,

    /// The location description of the device.
    pub location_description: Option<String>,

    /// The software build ID of the device.
    pub sw_build_id: Option<String>,
}

impl From<Box<[ReadAttributeResult<Id>]>> for Attributes {
    fn from(attributes: Box<[ReadAttributeResult<Id>]>) -> Self {
        let mut instance = Self::default();

        for attribute in attributes.into_iter().filter_map(|result| {
            result
                .inspect_err(|error| {
                    warn!("Invalid attribute: {error}");
                })
                .ok()
        }) {
            match attribute {
                Attribute::ZclVersion(version) => instance.zcl_version = version.into(),
                Attribute::ApplicationVersion(version) => {
                    instance.application_version = version.into();
                }
                Attribute::StackVersion(version) => instance.stack_version = version.into(),
                Attribute::HwVersion(version) => instance.hw_version = version.into(),
                Attribute::ManufacturerName(name) => match name.try_as_str() {
                    Ok(name) => instance.manufacturer_name = Some(name.to_string()),
                    Err(error) => warn!("Invalid manufacturer name: {error}"),
                },
                Attribute::ModelIdentifier(identifier) => match identifier.try_as_str() {
                    Ok(identifier) => instance.model_identifier = Some(identifier.to_string()),
                    Err(error) => warn!("Invalid model identifier: {error}"),
                },
                Attribute::DateCode(date_code) => instance.date_code = Some(date_code),
                Attribute::PowerSource(power_source) => instance.power_source = Some(power_source),
                Attribute::LocationDescription(location_description) => {
                    match location_description.try_as_str() {
                        Ok(location_description) => {
                            instance.location_description = Some(location_description.to_string());
                        }
                        Err(error) => warn!("Invalid location description: {error}"),
                    }
                }
                Attribute::SwBuildId(sw_build_id) => match sw_build_id.try_as_str() {
                    Ok(sw_build_id) => instance.sw_build_id = Some(sw_build_id.to_string()),
                    Err(error) => warn!("Invalid software build ID: {error}"),
                },
                other => trace!("Unexpected attribute: {other:?}"),
            }
        }

        instance
    }
}
