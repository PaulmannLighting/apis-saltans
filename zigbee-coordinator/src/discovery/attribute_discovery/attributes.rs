use log::{trace, warn};
use zcl::general::basic::readable::{Attribute, Id};
use zcl::general::basic::{DateCode, PowerSource};

use crate::ReadAttributeResult;

/// The attributes we want to discover.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Attributes {
    pub(crate) zcl_version: Option<u8>,
    pub(crate) application_version: Option<u8>,
    pub(crate) stack_version: Option<u8>,
    pub(crate) hw_version: Option<u8>,
    pub(crate) manufacturer_name: Option<String>,
    pub(crate) model_identifier: Option<String>,
    pub(crate) date_code: Option<DateCode>,
    pub(crate) power_source: Option<PowerSource>,
    pub(crate) location_description: Option<String>,
    pub(crate) sw_build_id: Option<String>,
}

impl Attributes {
    /// Get the ZCL version.
    #[must_use]
    pub const fn zcl_version(&self) -> Option<u8> {
        self.zcl_version
    }

    /// Get the application version.
    #[must_use]
    pub const fn application_version(&self) -> Option<u8> {
        self.application_version
    }

    /// Get the stack version.
    #[must_use]
    pub const fn stack_version(&self) -> Option<u8> {
        self.stack_version
    }

    /// Get the hardware version.
    #[must_use]
    pub const fn hw_version(&self) -> Option<u8> {
        self.hw_version
    }

    /// Get the manufacturer name.
    #[must_use]
    pub fn manufacturer_name(&self) -> Option<&str> {
        self.manufacturer_name.as_deref()
    }

    /// Get the model identifier.
    #[must_use]
    pub fn model_identifier(&self) -> Option<&str> {
        self.model_identifier.as_deref()
    }

    /// Get the date code.
    #[must_use]
    pub const fn date_code(&self) -> Option<&DateCode> {
        self.date_code.as_ref()
    }

    /// Get the power source.
    #[must_use]
    pub const fn power_source(&self) -> Option<PowerSource> {
        self.power_source
    }

    /// Get the location description.
    #[must_use]
    pub fn location_description(&self) -> Option<&str> {
        self.location_description.as_deref()
    }

    /// Get the software build ID.
    #[must_use]
    pub fn sw_build_id(&self) -> Option<&str> {
        self.sw_build_id.as_deref()
    }
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
