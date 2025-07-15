#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum General {
    Data8(u8),
    Data16([u8; 2]),
    Data24([u8; 3]),
    Data32([u8; 4]),
    Data40([u8; 5]),
    Data48([u8; 6]),
    Data56([u8; 7]),
    Data64([u8; 8]),
}
