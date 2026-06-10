//! Attributes for the occupancy sensing cluster.

use repr_discriminant::ReprDiscriminant;

pub mod occupancy_sensor_information_set;
pub mod physical_contact_configuration_set;
pub mod pir_configuration_set;
pub mod ultrasonic_configuration_set;

/// Attributes of the occupancy sensing cluster.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Attribute {
    /// Occupancy sensor attributes.
    Occupancy(occupancy_sensor_information_set::Attribute),
    /// PIR sensor attributes.
    Pir(pir_configuration_set::Attribute),
    /// Ultrasonic sensor attributes.
    Ultrasonic(ultrasonic_configuration_set::Attribute),
    /// Physical contact sensor attributes.
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
