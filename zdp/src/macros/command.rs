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

        impl zb_core::ClusterSpecific for $command {
            const ID: u16 = Self::ID;
        }

        impl zb_core::Profiled for $command {
            const PROFILE: zb_core::Profile = zb_core::Profile::Network;
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
        impl zb_core::ExpectResponse<$crate::services::Command> for $command {
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
