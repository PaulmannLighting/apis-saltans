use std::io::{Read, Seek};

pub(super) trait ImageSource: Read + Seek + Send {}

impl<T> ImageSource for T where T: Read + Seek + Send {}
