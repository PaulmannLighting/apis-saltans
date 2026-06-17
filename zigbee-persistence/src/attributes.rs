use serde::{Deserialize, Serialize};
use zcl::general::basic::{DateCode, PowerSource};

/// The attributes we want to discover.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Attributes {
    pub zcl_version: Option<u8>,
    pub application_version: Option<u8>,
    pub stack_version: Option<u8>,
    pub hw_version: Option<u8>,
    pub manufacturer_name: Option<String>,
    pub model_identifier: Option<String>,
    pub date_code: Option<DateCode>,
    pub power_source: Option<PowerSource>,
    pub location_description: Option<String>,
    pub sw_build_id: Option<String>,
}
