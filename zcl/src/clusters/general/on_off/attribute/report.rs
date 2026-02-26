//! Reportable attributes for the On/Off cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::{Bool, Type};

use crate::ParseAttributeError;
use crate::global::report_attributes::AttributeReport;

/// Readable attributes for the On/Off cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// On/Off state of the device.
    OnOff(Bool) = 0x0000,
}

impl From<Attribute> for AttributeReport {
    fn from(attribute: Attribute) -> Self {
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        // SAFETY: We provide the attribute's correct discriminant and appropriate `Type`.
        match attribute {
            Attribute::OnOff(on_off) => unsafe {
                Self::new(attribute.discriminant(), Type::Boolean(on_off))
            },
        }
    }
}

impl TryFrom<AttributeReport> for Attribute {
    type Error = ParseAttributeError;

    fn try_from(attribute_report: AttributeReport) -> Result<Self, Self::Error> {
        match attribute_report.attribute_id() {
            0x0000 => match attribute_report.into_data() {
                Type::Boolean(on_off) => Ok(Self::OnOff(on_off)),
                other => Err(ParseAttributeError::InvalidType(other)),
            },
            other => Err(ParseAttributeError::InvalidId(other)),
        }
    }
}
