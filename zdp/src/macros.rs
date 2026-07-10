//! ZDP service macros.

macro_rules! zdp_command {
    (
        $(#[$attribute:meta])*
        derive { $($extra_derive:path),* $(,)? }
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        $(response: $response:ty;)?
        fields {
            $($field:ident: $field_type:ty),* $(,)?
        }
        constructor {
            $($constructor:tt)*
        }
        getters {
            $($getter:tt)*
        }
        $(display {
            $($display:tt)*
        })?
        $(le_stream {
            $($le_stream:tt)*
        })?
        $(from {
            $($from:item)*
        })?
        $(try_from {
            $($try_from:item)*
        })?
    ) => {
        $crate::zdp_command! {
            @stream
            [$($attribute),*]
            [$($extra_derive),*]
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                $($constructor)*
            }
            getters {
                $($getter)*
            }
            display {
                $($($display)*)?
            }
            le_stream {
                $($($le_stream)*)?
            }
            from {
                $($($from)*)?
            }
            try_from {
                $($($try_from)*)?
            }
        }
    };
    (
        $(#[$attribute:meta])*
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        $(response: $response:ty;)?
        fields {
            $($field:ident: $field_type:ty),* $(,)?
        }
        constructor {
            $($constructor:tt)*
        }
        getters {
            $($getter:tt)*
        }
        $(display {
            $($display:tt)*
        })?
        $(le_stream {
            $($le_stream:tt)*
        })?
        $(from {
            $($from:item)*
        })?
        $(try_from {
            $($try_from:item)*
        })?
    ) => {
        $crate::zdp_command! {
            @stream
            [$($attribute),*]
            []
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                $($constructor)*
            }
            getters {
                $($getter)*
            }
            display {
                $($($display)*)?
            }
            le_stream {
                $($($le_stream)*)?
            }
            from {
                $($($from)*)?
            }
            try_from {
                $($($try_from)*)?
            }
        }
    };
    (
        $(#[$attribute:meta])*
        derive { $($extra_derive:path),* $(,)? }
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        $(response: $response:ty;)?
        fields {
            $($field:ident: $field_type:ty),* $(,)?
        }
        getters {
            $($getter:tt)*
        }
        $(display {
            $($display:tt)*
        })?
        $(le_stream {
            $($le_stream:tt)*
        })?
        $(from {
            $($from:item)*
        })?
        $(try_from {
            $($try_from:item)*
        })?
    ) => {
        $crate::zdp_command! {
            @stream
            [$($attribute),*]
            [$($extra_derive),*]
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                /// Creates a new instance.
                #[must_use]
                pub const fn new($($field: $field_type),*) -> Self {
                    Self {
                        $($field),*
                    }
                }
            }
            getters {
                $($getter)*
            }
            display {
                $($($display)*)?
            }
            le_stream {
                $($($le_stream)*)?
            }
            from {
                $($($from)*)?
            }
            try_from {
                $($($try_from)*)?
            }
        }
    };
    (
        $(#[$attribute:meta])*
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        $(response: $response:ty;)?
        fields {
            $($field:ident: $field_type:ty),* $(,)?
        }
        getters {
            $($getter:tt)*
        }
        $(display {
            $($display:tt)*
        })?
        $(le_stream {
            $($le_stream:tt)*
        })?
        $(from {
            $($from:item)*
        })?
        $(try_from {
            $($try_from:item)*
        })?
    ) => {
        $crate::zdp_command! {
            @stream
            [$($attribute),*]
            []
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                /// Creates a new instance.
                #[must_use]
                pub const fn new($($field: $field_type),*) -> Self {
                    Self {
                        $($field),*
                    }
                }
            }
            getters {
                $($getter)*
            }
            display {
                $($($display)*)?
            }
            le_stream {
                $($($le_stream)*)?
            }
            from {
                $($($from)*)?
            }
            try_from {
                $($($try_from)*)?
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        response [$($response:ty)?];
        fields {
            $($field:ident: $field_type:ty),*
        }
        constructor {
            $($constructor:tt)*
        }
        getters {
            $($getter:tt)*
        }
        display {
            $($display:tt)*
        }
        le_stream {
        }
        from {
            $($from:item)*
        }
        try_from {
            $($try_from:item)*
        }
    ) => {
        $crate::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            [le_stream::FromLeStream, le_stream::ToLeStream]
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                $($constructor)*
            }
            getters {
                $($getter)*
            }
            display {
                $($display)*
            }
            from_le_stream {
            }
            to_le_stream {
            }
            from {
                $($from)*
            }
            try_from {
                $($try_from)*
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        response [$($response:ty)?];
        fields {
            $($field:ident: $field_type:ty),*
        }
        constructor {
            $($constructor:tt)*
        }
        getters {
            $($getter:tt)*
        }
        display {
            $($display:tt)*
        }
        le_stream {
            from {
                $($from_le_stream:tt)*
            }
        }
        from {
            $($from:item)*
        }
        try_from {
            $($try_from:item)*
        }
    ) => {
        $crate::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            [le_stream::ToLeStream]
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                $($constructor)*
            }
            getters {
                $($getter)*
            }
            display {
                $($display)*
            }
            from_le_stream {
                $($from_le_stream)*
            }
            to_le_stream {
            }
            from {
                $($from)*
            }
            try_from {
                $($try_from)*
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        response [$($response:ty)?];
        fields {
            $($field:ident: $field_type:ty),*
        }
        constructor {
            $($constructor:tt)*
        }
        getters {
            $($getter:tt)*
        }
        display {
            $($display:tt)*
        }
        le_stream {
            to {
                $($to_le_stream:tt)*
            }
        }
        from {
            $($from:item)*
        }
        try_from {
            $($try_from:item)*
        }
    ) => {
        $crate::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            [le_stream::FromLeStream]
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                $($constructor)*
            }
            getters {
                $($getter)*
            }
            display {
                $($display)*
            }
            from_le_stream {
            }
            to_le_stream {
                $($to_le_stream)*
            }
            from {
                $($from)*
            }
            try_from {
                $($try_from)*
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        response [$($response:ty)?];
        fields {
            $($field:ident: $field_type:ty),*
        }
        constructor {
            $($constructor:tt)*
        }
        getters {
            $($getter:tt)*
        }
        display {
            $($display:tt)*
        }
        le_stream {
            from {
                $($from_le_stream:tt)*
            }
            to {
                $($to_le_stream:tt)*
            }
        }
        from {
            $($from:item)*
        }
        try_from {
            $($try_from:item)*
        }
    ) => {
        $crate::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            []
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                $($constructor)*
            }
            getters {
                $($getter)*
            }
            display {
                $($display)*
            }
            from_le_stream {
                $($from_le_stream)*
            }
            to_le_stream {
                $($to_le_stream)*
            }
            from {
                $($from)*
            }
            try_from {
                $($try_from)*
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        response [$($response:ty)?];
        fields {
            $($field:ident: $field_type:ty),*
        }
        constructor {
            $($constructor:tt)*
        }
        getters {
            $($getter:tt)*
        }
        display {
            $($display:tt)*
        }
        le_stream {
            to {
                $($to_le_stream:tt)*
            }
            from {
                $($from_le_stream:tt)*
            }
        }
        from {
            $($from:item)*
        }
        try_from {
            $($try_from:item)*
        }
    ) => {
        $crate::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            []
            $command => $name;
            cluster_id: $cluster_id;
            group: $group;
            response [$($response)?];
            fields {
                $($field: $field_type),*
            }
            constructor {
                $($constructor)*
            }
            getters {
                $($getter)*
            }
            display {
                $($display)*
            }
            from_le_stream {
                $($from_le_stream)*
            }
            to_le_stream {
                $($to_le_stream)*
            }
            from {
                $($from)*
            }
            try_from {
                $($try_from)*
            }
        }
    };
    (
        @emit
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        [$($stream_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
        group: $group:ident;
        response [$($response:ty)?];
        fields {
            $($field:ident: $field_type:ty),*
        }
        constructor {
            $($constructor:tt)*
        }
        getters {
            $($getter:tt)*
        }
        display {
            $($display:tt)*
        }
        from_le_stream {
            $($from_le_stream:tt)*
        }
        to_le_stream {
            $($to_le_stream:tt)*
        }
        from {
            $($from:item)*
        }
        try_from {
            $($try_from:item)*
        }
    ) => {
        $(#[$attribute])*
        #[derive(Clone, Debug, Eq, PartialEq, Hash $(, $extra_derive)* $(, $stream_derive)*)]
        pub struct $command {
            $($field: $field_type),*
        }

        impl $command {
            /// The cluster ID.
            pub const ID: u16 = $cluster_id;

            /// The service name.
            pub const NAME: &'static str = stringify!($name);

            $($constructor)*

            $($getter)*
        }

        impl apis_saltans_core::ClusterSpecific for $command {
            const ID: u16 = Self::ID;
        }

        impl apis_saltans_core::Profiled for $command {
            const PROFILE: apis_saltans_core::Profile = apis_saltans_core::Profile::Network;
        }

        impl $crate::services::Service for $command {
            const NAME: &'static str = Self::NAME;
        }

        impl From<Box<Self>> for $command {
            fn from(value: Box<Self>) -> Self {
                *value
            }
        }

        impl From<$command> for $crate::Command {
            fn from(value: $command) -> Self {
                Self::$group(value.into())
            }
        }

        impl TryFrom<$crate::Command> for $command {
            type Error = $crate::Command;

            fn try_from(command: $crate::Command) -> Result<Self, Self::Error> {
                if let $crate::Command::$group($crate::$group::$command(command_struct)) = command {
                    Ok(command_struct.into())
                } else {
                    Err(command)
                }
            }
        }

        $crate::zdp_command! {
            @response
            $command
            $($response)?
        }

        impl std::fmt::Display for $command {
            $crate::zdp_command! {
                @display
                self,
                f,
                [$($field),*],
                {
                    $($display)*
                }
            }
        }

        $($from)*

        $($try_from)*

        $crate::zdp_command! {
            @from_le_stream
            $command
            {
                $($from_le_stream)*
            }
        }

        $crate::zdp_command! {
            @to_le_stream
            $command
            {
                $($to_le_stream)*
            }
        }
    };
    (
        @from_le_stream
        $command:ident
        {
        }
    ) => {};
    (
        @from_le_stream
        $command:ident
        {
            $($from_le_stream:tt)+
        }
    ) => {
        impl le_stream::FromLeStream for $command {
            $($from_le_stream)+
        }
    };
    (
        @to_le_stream
        $command:ident
        {
        }
    ) => {};
    (
        @to_le_stream
        $command:ident
        {
            $($to_le_stream:tt)+
        }
    ) => {
        impl le_stream::ToLeStream for $command {
            $($to_le_stream)+
        }
    };
    (@response $command:ident) => {};
    (@response $command:ident $response:ty) => {
        impl apis_saltans_core::ExpectResponse<$crate::services::Command> for $command {
            type Response = $response;
        }
    };
    (
        @display
        $self:ident,
        $formatter:ident,
        [$($field:ident),*],
        {
        }
    ) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut debug = f.debug_struct(Self::NAME);
            $(debug.field(stringify!($field), &self.$field);)*
            debug.finish()
        }
    };
    (
        @display
        $self:ident,
        $formatter:ident,
        [$($field:ident),*],
        {
            $($display:tt)+
        }
    ) => {
        $($display)+
    };
}

pub(crate) use zdp_command;

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
                &[$(<$command as apis_saltans_core::ClusterSpecific>::ID),*]
            }

            /// Returns the cluster ID of the command.
            #[must_use]
            pub const fn cluster_id(&self) -> u16 {
                match self {
                    $(Self::$command(_) => <$command as apis_saltans_core::ClusterSpecific>::ID),*
                }
            }

            /// Returns the profile of the command.
            #[must_use]
            pub const fn profile(&self) -> apis_saltans_core::Profile {
                match self {
                    $(Self::$command(_) => <$command as apis_saltans_core::Profiled>::PROFILE),*
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
                        <$command as apis_saltans_core::ClusterSpecific>::ID => {
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
            pub const fn profile(&self) -> apis_saltans_core::Profile {
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
