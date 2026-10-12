//! Attribute value types of the IAS Zone cluster.

use zb_core::IeeeAddress;
use zb_core::types::Uint8;

use crate::macros::zcl_attribute_newtype;
use crate::{Discrete, ReportingType};

zcl_attribute_newtype! {
    /// The zone state.
    pub struct ZoneState(Uint8) => Enum8;
}

zcl_attribute_newtype! {
    /// The address of the IAS CIE device.
    pub struct IasCieAddress(IeeeAddress) => IeeeAddress;
}

impl ReportingType for IasCieAddress {
    type Configuration = Discrete<Self>;
}
