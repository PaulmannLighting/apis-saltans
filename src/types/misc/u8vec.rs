use crate::constants::U8_CAPACITY;

/// A type representing an array of dynamic size with a fixed capacity.
pub type U8Vec<T> = heapless::Vec<T, U8_CAPACITY>;
