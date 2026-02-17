//! Attributes for the occupancy sensing cluster.

use le_stream::{FromLeStream, FromLeStreamTagged};
use repr_discriminant::ReprDiscriminant;

pub mod occupancy_sensor_information_set;
pub mod physical_contact_configuration_set;
pub mod pir_configuration_set;
pub mod ultrasonic_configuration_set;

pub enum Attribute {
    Occupancy(occupancy_sensor_information_set::Attribute),
    Pir(pir_configuration_set::Attribute),
    Ultrasonic(ultrasonic_configuration_set::Attribute),
    PhysicalContact(physical_contact_configuration_set::Attribute),
}

#[expect(unsafe_code)]
// SAFETY: All variants safely derive `ReprDiscriminant`.
unsafe impl ReprDiscriminant for Attribute {
    type Repr = u16;

    fn repr_discriminant(&self) -> Self::Repr {
        match self {
            Self::Occupancy(attr) => attr.discriminant(),
            Self::Pir(attr) => attr.discriminant(),
            Self::Ultrasonic(attr) => attr.discriminant(),
            Self::PhysicalContact(attr) => attr.discriminant(),
        }
    }
}

impl FromLeStreamTagged for Attribute {
    type Tag = u16;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        match tag {
            0x0000..=0x0002 => {
                occupancy_sensor_information_set::Attribute::from_le_stream_tagged(tag, bytes)
                    .map(|opt| opt.map(Self::Occupancy))
            }
            0x0010..=0x0012 => pir_configuration_set::Attribute::from_le_stream_tagged(tag, bytes)
                .map(|opt| opt.map(Self::Pir)),
            0x0020..=0x0022 => {
                ultrasonic_configuration_set::Attribute::from_le_stream_tagged(tag, bytes)
                    .map(|opt| opt.map(Self::Ultrasonic))
            }
            0x0030..=0x0032 => {
                physical_contact_configuration_set::Attribute::from_le_stream_tagged(tag, bytes)
                    .map(|opt| opt.map(Self::PhysicalContact))
            }
            other => Err(other),
        }
    }
}
