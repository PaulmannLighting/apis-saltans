macro_rules! zdp_command_group {
    (
        $(#[$attribute:meta])*
        $group:ident {
            $($command:ident),* $(,)?
        }
    ) => {
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $group {
            $(
                #[doc = concat!("Command variant for `", stringify!($command), "`.")]
                $command(Box<$command>)
            ),*
        }

        impl $group {
            /// Returns all cluster IDs supported by this command group.
            pub(crate) const fn cluster_ids() -> &'static [u16] {
                &[$(<$command as zb_core::ClusterSpecific>::ID),*]
            }

            /// Returns the cluster ID of the command.
            #[must_use]
            pub const fn cluster_id(&self) -> u16 {
                match self {
                    $(Self::$command(_) => <$command as zb_core::ClusterSpecific>::ID),*
                }
            }

            /// Returns the profile of the command.
            #[must_use]
            pub const fn profile(&self) -> zb_core::Profile {
                match self {
                    $(Self::$command(_) => <$command as zb_core::Profiled>::PROFILE),*
                }
            }

            /// Parses a command from the given cluster ID and byte stream.
            pub(crate) fn parse_with_cluster_id<T>(
                cluster_id: u16,
                bytes: T,
            ) -> Result<Option<Self>, u16>
            where
                T: Iterator<Item = u8>,
            {
                match cluster_id {
                    $(
                        <$command as zb_core::ClusterSpecific>::ID => {
                            Ok(<$command as le_stream::FromLeStream>::from_le_stream(bytes)
                                .map(Self::from))
                        },
                    )*
                    other => Err(other),
                }
            }
        }

        impl std::fmt::Display for $group {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$command(command) => std::fmt::Display::fmt(command, f)),*
                }
            }
        }

        impl le_stream::ToLeStream for $group {
            type Iter = to_le_stream::Iter;

            fn to_le_stream(self) -> Self::Iter {
                match self {
                    $(Self::$command(command) => {
                        to_le_stream::Iter::$command(
                            <$command as le_stream::ToLeStream>::to_le_stream(*command)
                                .into()
                        )
                    }),*
                }
            }
        }

        /// Little-endian stream iterators for command group variants.
        pub mod to_le_stream {
            /// Little-endian stream iterator for a command group.
            #[allow(clippy::enum_variant_names)]
            #[derive(Debug)]
            pub enum Iter {
                $(
                    #[doc = concat!(
                        "Little-endian stream iterator for `",
                        stringify!($command),
                        "`."
                    )]
                    $command(Box<<super::$command as le_stream::ToLeStream>::Iter>)
                ),*
            }

            impl Iterator for Iter {
                type Item = u8;

                fn next(&mut self) -> Option<Self::Item> {
                    #[allow(clippy::match_same_arms)]
                    match self {
                        $(Self::$command(iter) => iter.next()),*
                    }
                }
            }
        }

        $(
            impl From<$command> for $group {
                fn from(command: $command) -> Self {
                    Self::$command(command.into())
                }
            }

            impl TryFrom<$group> for $command {
                type Error = $group;

                fn try_from(command: $group) -> Result<Self, Self::Error> {
                    match command {
                        $group::$command(command) => Ok(*command),
                        other => Err(other),
                    }
                }
            }
        )*
    };
}

pub(crate) use zdp_command_group;
