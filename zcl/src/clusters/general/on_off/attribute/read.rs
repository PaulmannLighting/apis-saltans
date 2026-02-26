//! Readable attributes for the On/Off cluster.

use le_stream::FromLeStreamTagged;
use repr_discriminant::ReprDiscriminant;
use zigbee::Parsable;
use zigbee::types::{Bool, Type, Uint16};

use super::StartUpOnOff;
use crate::global::read_attributes::ReadAttributesStatus;

/// Readable attributes for the On/Off cluster.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
#[derive(ReprDiscriminant, FromLeStreamTagged)]
pub enum Attribute {
    /// On/Off state of the device.
    OnOff(Bool) = 0x0000,
    ///  Global scene control.
    GlobalSceneControl(Bool) = 0x4000,
    ///  On time attribute.
    OnTime(Uint16) = 0x4001,
    ///  Off wait time attribute.
    OffWaitTime(Uint16) = 0x4002,
    ///  Start up on off attribute.
    StartUpOnOff(Parsable<u8, StartUpOnOff>) = 0x4003,
}

impl From<Attribute> for ReadAttributesStatus {
    fn from(attribute: Attribute) -> Self {
        #[expect(unsafe_code, clippy::undocumented_unsafe_blocks)]
        // SAFETY: We provide the attribute's correct discriminant and appropriate `Type`.
        match attribute {
            Attribute::OnOff(on_off) => unsafe {
                Self::new(attribute.discriminant(), Ok(Type::Boolean(on_off)))
            },
            Attribute::GlobalSceneControl(gsc) => unsafe {
                Self::new(attribute.discriminant(), Ok(Type::Boolean(gsc)))
            },
            Attribute::OnTime(on_time) => unsafe {
                Self::new(attribute.discriminant(), Ok(Type::Uint16(on_time)))
            },
            Attribute::OffWaitTime(off_time) => unsafe {
                Self::new(attribute.discriminant(), Ok(Type::Uint16(off_time)))
            },
            Attribute::StartUpOnOff(parsable) => unsafe {
                Self::new(
                    attribute.discriminant(),
                    Ok(Type::Map8(parsable.into_src())),
                )
            },
        }
    }
}

impl TryFrom<ReadAttributesStatus> for Attribute {
    type Error = ReadAttributesStatus;

    fn try_from(read_attributes_status: ReadAttributesStatus) -> Result<Self, Self::Error> {
        match read_attributes_status.into_parts() {
            (0x0000, Ok(Type::Boolean(on_off))) => Ok(Self::OnOff(on_off)),
            (0x4000, Ok(Type::Boolean(gsc))) => Ok(Self::GlobalSceneControl(gsc)),
            (0x4001, Ok(Type::Uint16(on_time))) => Ok(Self::OnTime(on_time)),
            (0x4002, Ok(Type::Uint16(off_time))) => Ok(Self::OffWaitTime(off_time)),
            (0x4003, Ok(Type::Map8(startup_on_off))) => {
                Ok(Self::StartUpOnOff(Parsable::new(startup_on_off)))
            }
            (id, typ) => {
                #[expect(unsafe_code)]
                // SAFETY: We reconstruct the original `ReadAttributeStatus` value.
                Err(unsafe { ReadAttributesStatus::new(id, typ) })
            }
        }
    }
}
