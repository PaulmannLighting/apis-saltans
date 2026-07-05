use crate::macros::zcl_attribute_newtype;

zcl_attribute_newtype! {
    /// Occupancy status as reported by the sensor.
    pub bitflags Occupancy(u8) => Map8 {
        /// Flag, whether the sensor detected an occupation.
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
