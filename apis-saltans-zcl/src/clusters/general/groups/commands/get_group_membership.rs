use core::iter::Chain;
use std::boxed::Box;

use apis_saltans_core::types::{Uint8, Uint16};
use apis_saltans_core::{ClusterId, Direction};
use le_stream::ToLeStream;

use crate::macros::zcl_command;

zcl_command! {
    /// Command to request the membership of a device in multiple groups.
    GetGroupMembership {
        { ClusterId::Groups } => Groups;
        command_id: 0x02;
        direction: Direction::ClientToServer;
        => super::GetGroupMembership;
        fields {
            groups: Box<[Uint16]>,
        }

        getters {
            /// Return the groups the sender is a member of.
            #[must_use]
            pub fn groups(&self) -> &[Uint16] {
                &self.groups
            }

            /// Return the group count.
            ///
            /// # Panics
            ///
            /// This function will panic if the number of groups exceeds [`Uint8::MAX`], which should never happen.
            #[must_use]
            pub fn group_count(&self) -> Uint8 {
                self.groups
                    .len()
                    .try_into()
                    .expect("GroupList size always fits into a Uint8.")
            }
        }

        from_le_stream {
            fn from_le_stream<I>(mut bytes: I) -> Option<Self>
                where
                    I: Iterator<Item = u8>,
                {
                    let group_count = Uint8::from_le_stream(&mut bytes)?;

                    let Ok(size) = u8::try_from(group_count) else {
                        return None;
                    };

                    let mut groups = Vec::with_capacity(size.into());

                    for _ in 0..size {
                        groups.push(Uint16::from_le_stream(&mut bytes)?);
                    }

                    Some(Self {
                        groups: groups.into_boxed_slice(),
                    })
                }
        }

        to_le_stream {
            type Iter = Chain<<Uint8 as ToLeStream>::Iter, <Box<[Uint16]> as ToLeStream>::Iter>;

                fn to_le_stream(self) -> Self::Iter {
                    self.group_count()
                        .to_le_stream()
                        .chain(self.groups.to_le_stream())
                }
        }
    }
}

impl AsRef<[Uint16]> for GetGroupMembership {
    fn as_ref(&self) -> &[Uint16] {
        &self.groups
    }
}

impl IntoIterator for GetGroupMembership {
    type Item = Uint16;
    type IntoIter = <Box<[Uint16]> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.groups.into_iter()
    }
}
