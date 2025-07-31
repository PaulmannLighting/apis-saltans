use crate::constants::U8_CAPACITY;

/// A type representing a string of dynamic size with a fixed capacity.
pub type U8String = heapless::String<U8_CAPACITY>;
