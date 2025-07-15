#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum String {
    OctStr(Vec<u8>),
    String(std::string::String),
    OctStr16(Vec<u8>),
    String16(std::string::String),
}
