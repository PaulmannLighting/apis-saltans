use heapless::CapacityError;
use macaddr::MacAddr8;

use crate::ByteSizedVec;

crate::services::zdp_command! {
    /// Parent Announcement Service.
    ParentAnnce => Parent_annce;
    cluster_id: 0x001F;
    group: DeviceAndServiceDiscovery;
    fields {
        child_info: ByteSizedVec<MacAddr8>,
    }
    getters {
        /// Returns a reference to the child info.
        #[must_use]
        pub fn child_info(&self) -> &[MacAddr8] {
            &self.child_info
        }
    }
    display {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {{ child_info: [", Self::NAME)?;

            let mut mac_addresses = self.child_info().iter();

            if let Some(mac_address) = mac_addresses.next() {
                write!(f, "{mac_address}")?;

                for mac_address in mac_addresses {
                    write!(f, ", {mac_address}")?;
                }
            }

            write!(f, "] }}")
        }
    }

    try_from {
        impl TryFrom<Box<[MacAddr8]>> for ParentAnnce {
            type Error = CapacityError;

            fn try_from(value: Box<[MacAddr8]>) -> Result<Self, Self::Error> {
                Self::try_from(&*value)
            }
        }

        impl TryFrom<Vec<MacAddr8>> for ParentAnnce {
            type Error = CapacityError;

            fn try_from(value: Vec<MacAddr8>) -> Result<Self, Self::Error> {
                Self::try_from(value.into_boxed_slice())
            }
        }

        impl TryFrom<&[MacAddr8]> for ParentAnnce {
            type Error = CapacityError;

            fn try_from(value: &[MacAddr8]) -> Result<Self, Self::Error> {
                value.try_into().map(Self::new)
            }
        }
    }
}
