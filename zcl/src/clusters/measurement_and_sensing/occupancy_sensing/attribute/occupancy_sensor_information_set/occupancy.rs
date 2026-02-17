use bitflags::bitflags;
use le_stream::{FromLeStream, ToLeStream};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream,
)]
pub struct Occupancy(u8);

bitflags! {
    impl Occupancy: u8 {
        const OCCUPIED = 0b0000_0001;
    }
}

impl From<bool> for Occupancy {
    fn from(occupied: bool) -> Self {
        if occupied {
            Self::OCCUPIED
        } else {
            Self::empty()
        }
    }
}

impl From<Occupancy> for bool {
    fn from(occ: Occupancy) -> Self {
        occ.contains(Occupancy::OCCUPIED)
    }
}
