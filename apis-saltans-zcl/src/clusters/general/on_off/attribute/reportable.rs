//! Reportable attributes for the On/Off cluster.

use le_stream::FromLeStream;
use repr_discriminant::ReprDiscriminant;
use apis_saltans_core::types::{Bool, Type};

use crate::global::report_attributes::AttributeReport;

/// Readable attributes for the On/Off cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStream)]
pub enum Attribute {
    /// On/Off state of the device.
    OnOff(Bool) = 0x0000,
}

impl From<Attribute> for AttributeReport {
    fn from(attribute: Attribute) -> Self {
        match attribute {
            Attribute::OnOff(on_off) => {
                #[expect(unsafe_code)]
                // SAFETY: We provide the attribute's correct discriminant and appropriate `Type`.
                unsafe {
                    Self::new(attribute.discriminant(), Type::Boolean(on_off))
                }
            }
        }
    }
}

impl TryFrom<AttributeReport> for Attribute {
    type Error = AttributeReport;

    fn try_from(attribute_report: AttributeReport) -> Result<Self, Self::Error> {
        match attribute_report.into_parts() {
            (0x0000, Type::Boolean(on_off)) => Ok(Self::OnOff(on_off)),
            (id, typ) => Err(
                #[expect(unsafe_code)]
                // SAFETY: We reconstruct the original `AttributeReport`.
                unsafe {
                    AttributeReport::new(id, typ)
                },
            ),
        }
    }
}
