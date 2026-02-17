//! Attributes for the occupancy sensing cluster.

pub mod occupancy_sensor_information_set;
pub mod pir_configuration_set;

pub enum Attribute {
    Occupancy(occupancy_sensor_information_set::Attribute),
    Pir(pir_configuration_set::Attribute),
}
