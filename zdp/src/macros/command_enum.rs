macro_rules! zdp_command_enum {
    (
        $(#[$attribute:meta])*
        $command_enum:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $command_enum {
            $(
                #[doc = concat!("Command variant for `", stringify!($variant), "`.")]
                $variant($variant)
            ),*
        }

        impl $command_enum {
            /// Parses a ZDP command from the given cluster ID and byte stream.
            pub(crate) fn parse_with_cluster_id<T>(
                cluster_id: u16,
                bytes: T,
            ) -> Result<Option<Self>, u16>
            where
                T: Iterator<Item = u8>,
            {
                $(
                    if $variant::cluster_ids().contains(&cluster_id) {
                        return $variant::parse_with_cluster_id(cluster_id, bytes)
                            .map(|command| command.map(Self::$variant));
                    }
                )*

                Err(cluster_id)
            }

            /// Return the cluster ID of the command.
            #[must_use]
            pub const fn cluster_id(&self) -> u16 {
                match self {
                    $(Self::$variant(command) => command.cluster_id()),*
                }
            }

            /// Return the profile of the command.
            #[must_use]
            pub const fn profile(&self) -> zb_core::Profile {
                match self {
                    $(Self::$variant(command) => command.profile()),*
                }
            }
        }

        impl std::fmt::Display for $command_enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(command) => std::fmt::Display::fmt(command, f)),*
                }
            }
        }

        impl le_stream::ToLeStream for $command_enum {
            type Iter = command_to_le_stream::Iter;

            fn to_le_stream(self) -> Self::Iter {
                match self {
                    $(Self::$variant(command) => {
                        command_to_le_stream::Iter::$variant(
                            <$variant as le_stream::ToLeStream>::to_le_stream(command)
                                .into()
                        )
                    }),*
                }
            }
        }

        /// Little-endian stream iterators for command variants.
        pub mod command_to_le_stream {
            /// Little-endian stream iterator for a command.
            #[derive(Debug)]
            pub enum Iter {
                $(
                    #[doc = concat!(
                        "Little-endian stream iterator for `",
                        stringify!($variant),
                        "`."
                    )]
                    $variant(Box<<super::$variant as le_stream::ToLeStream>::Iter>)
                ),*
            }

            impl Iterator for Iter {
                type Item = u8;

                fn next(&mut self) -> Option<Self::Item> {
                    #[allow(clippy::match_same_arms)]
                    match self {
                        $(Self::$variant(iter) => iter.next()),*
                    }
                }
            }
        }

        $(
            impl From<$variant> for $command_enum {
                fn from(command: $variant) -> Self {
                    Self::$variant(command)
                }
            }

            impl TryFrom<$command_enum> for $variant {
                type Error = $command_enum;

                fn try_from(command: $command_enum) -> Result<Self, Self::Error> {
                    match command {
                        $command_enum::$variant(command) => Ok(command),
                        other => Err(other),
                    }
                }
            }
        )*
    };
}

pub(crate) use zdp_command_enum;
