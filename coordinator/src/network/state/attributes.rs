use log::{trace, warn};
use serde::{Deserialize, Serialize};
use zb_zcl::basic::{DateCode, Id, PowerSource, Readable};

use crate::ReadAttributeResult;

/// The attributes we want to discover.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize, Serialize)]
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

impl Attributes {
    /// Return the ZCL version of the device.
    #[must_use]
    pub const fn zcl_version(&self) -> Option<u8> {
        self.zcl_version
    }

    /// Replace the ZCL version of the device.
    pub const fn set_zcl_version(&mut self, zcl_version: u8) -> Option<u8> {
        self.zcl_version.replace(zcl_version)
    }

    /// Return the application version of the device.
    #[must_use]
    pub const fn application_version(&self) -> Option<u8> {
        self.application_version
    }

    /// Replace the application version of the device.
    pub const fn set_application_version(&mut self, application_version: u8) -> Option<u8> {
        self.application_version.replace(application_version)
    }

    /// Return the stack version of the device.
    #[must_use]
    pub const fn stack_version(&self) -> Option<u8> {
        self.stack_version
    }

    /// Replace the stack version of the device.
    pub const fn set_stack_version(&mut self, stack_version: u8) -> Option<u8> {
        self.stack_version.replace(stack_version)
    }

    /// Return the hardware version of the device.
    #[must_use]
    pub const fn hw_version(&self) -> Option<u8> {
        self.hw_version
    }

    /// Replace the hardware version of the device.
    pub const fn set_hw_version(&mut self, hw_version: u8) -> Option<u8> {
        self.hw_version.replace(hw_version)
    }

    /// Return the manufacturer name of the device.
    #[must_use]
    pub fn manufacturer_name(&self) -> Option<&str> {
        self.manufacturer_name.as_deref()
    }

    /// Replace the manufacturer name of the device.
    pub const fn set_manufacturer_name(&mut self, manufacturer_name: String) -> Option<String> {
        self.manufacturer_name.replace(manufacturer_name)
    }

    /// Return the model identifier of the device.
    #[must_use]
    pub fn model_identifier(&self) -> Option<&str> {
        self.model_identifier.as_deref()
    }

    /// Replace the model identifier of the device.
    pub const fn set_model_identifier(&mut self, model_identifier: String) -> Option<String> {
        self.model_identifier.replace(model_identifier)
    }

    /// Return the date code of the device.
    #[must_use]
    pub const fn date_code(&self) -> Option<&DateCode> {
        self.date_code.as_ref()
    }

    /// Replace the date code of the device.
    pub const fn set_date_code(&mut self, date_code: DateCode) -> Option<DateCode> {
        self.date_code.replace(date_code)
    }

    /// Return the power source of the device.
    #[must_use]
    pub const fn power_source(&self) -> Option<&PowerSource> {
        self.power_source.as_ref()
    }

    /// Replace the power source of the device.
    pub const fn set_power_source(&mut self, power_source: PowerSource) -> Option<PowerSource> {
        self.power_source.replace(power_source)
    }

    /// Return the location description of the device.
    #[must_use]
    pub fn location_description(&self) -> Option<&str> {
        self.location_description.as_deref()
    }

    /// Replace the location description of the device.
    pub const fn set_location_description(
        &mut self,
        location_description: String,
    ) -> Option<String> {
        self.location_description.replace(location_description)
    }

    /// Return the software build ID of the device.
    #[must_use]
    pub fn sw_build_id(&self) -> Option<&str> {
        self.sw_build_id.as_deref()
    }

    /// Replace the software build ID of the device.
    pub const fn set_sw_build_id(&mut self, sw_build_id: String) -> Option<String> {
        self.sw_build_id.replace(sw_build_id)
    }
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
                Readable::ZclVersion(version) => instance.zcl_version = version.into(),
                Readable::ApplicationVersion(version) => {
                    instance.application_version = version.into();
                }
                Readable::StackVersion(version) => instance.stack_version = version.into(),
                Readable::HwVersion(version) => instance.hw_version = version.into(),
                Readable::ManufacturerName(name) => match name.try_as_str() {
                    Ok(name) => instance.manufacturer_name = Some(name.to_string()),
                    Err(error) => warn!("Invalid manufacturer name: {error}"),
                },
                Readable::ModelIdentifier(identifier) => match identifier.try_as_str() {
                    Ok(identifier) => instance.model_identifier = Some(identifier.to_string()),
                    Err(error) => warn!("Invalid model identifier: {error}"),
                },
                Readable::DateCode(date_code) => instance.date_code = Some(date_code),
                Readable::PowerSource(power_source) => instance.power_source = Some(power_source),
                Readable::LocationDescription(location_description) => {
                    match location_description.try_as_str() {
                        Ok(location_description) => {
                            instance.location_description = Some(location_description.to_string());
                        }
                        Err(error) => warn!("Invalid location description: {error}"),
                    }
                }
                Readable::SwBuildId(sw_build_id) => match sw_build_id.try_as_str() {
                    Ok(sw_build_id) => instance.sw_build_id = Some(sw_build_id.to_string()),
                    Err(error) => warn!("Invalid software build ID: {error}"),
                },
                other => trace!("Unexpected attribute: {other:?}"),
            }
        }

        instance
    }
}
