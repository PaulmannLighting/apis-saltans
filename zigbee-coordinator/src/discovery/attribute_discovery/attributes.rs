use log::{trace, warn};
use zcl::general::basic::readable::{Attribute, Id};
use zcl::general::basic::{DateCode, PowerSource};

use crate::ReadAttributeResult;

/// The attributes we want to discover.
#[derive(Debug)]
pub struct Attributes {
    zcl_version: Option<u8>,
    application_version: Option<u8>,
    stack_version: Option<u8>,
    hw_version: Option<u8>,
    manufacturer_name: Option<String>,
    model_identifier: Option<String>,
    date_code: Option<DateCode>,
    power_source: Option<PowerSource>,
    location_description: Option<String>,
    sw_build_id: Option<String>,
}

impl From<Box<[ReadAttributeResult<Id>]>> for Attributes {
    fn from(attributes: Box<[ReadAttributeResult<Id>]>) -> Self {
        let mut instance = Self {
            zcl_version: None,
            application_version: None,
            stack_version: None,
            hw_version: None,
            manufacturer_name: None,
            model_identifier: None,
            date_code: None,
            power_source: None,
            location_description: None,
            sw_build_id: None,
        };

        for attribute in attributes.into_iter().filter_map(Result::ok) {
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
