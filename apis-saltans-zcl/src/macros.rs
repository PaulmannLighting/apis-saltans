/// Define a ZCL command payload and implement its common command traits.
///
/// Cluster-specific commands use the cluster ID to implement
/// [`apis_saltans_core::Cluster`]:
///
/// ```ignore
/// zcl_command! {
///     /// Switch a device on.
///     On {
///         { ClusterId::OnOff } => OnOff;
///         command_id: 0x01;
///         direction: Direction::ClientToServer;
///         => super::On;
///         fields;
///     }
/// }
/// ```
///
/// Global commands omit the cluster ID and implement [`crate::command::Scoped`]
/// with [`crate::Scope::Global`] instead. A `constructor` section can override
/// the generated `new` constructor, and a `getters` section can contain
/// accessor methods. Optional `from_le_stream` and `to_le_stream` sections
/// replace the respective derive, and the final `impl` section can contain
/// custom inherent or trait implementations for the type:
///
/// ```ignore
/// zcl_command! {
///     /// Read Attributes command.
///     Command {
///         Global;
///         command_id: 0x00;
///         direction: Direction::ClientToServer;
///         response: Response;
///         => crate::global::ReadAttributes;
///         derive(Ord, PartialOrd);
///         fields {
///             attribute_ids: Box<[u16]>,
///         }
///
///         constructor {
///             pub const fn new(attribute_ids: Box<[u16]>) -> Self {
///                 Self { attribute_ids }
///             }
///         }
///
///         getters {
///             pub fn attribute_ids(&self) -> &[u16] {
///                 &self.attribute_ids
///             }
///         }
///
///         impl {
///             impl Command {
///                 // Custom inherent methods go here.
///             }
///         }
///     }
/// }
/// ```
macro_rules! zcl_command {
    (
        $(#[$attr:meta])*
        $command:ident {
            { $cluster_id:expr } => $cluster_variant:ident;
            command_id: $command_id:expr;
            direction: $direction:expr;
            $(disable_default_response: $disable_default_response:expr;)?
            $(response: $response:ty;)?
            => $try_module:ident::$try_variant_or_module:ident $(::$try_variant:ident)?;
            $(derive($($extra_derive:path),* $(,)?);)?
            fields;
            $($rest:tt)*
        }
    ) => {
        $crate::macros::zcl_command! {
            @parse_constructor
            [unit]
            [$(#[$attr])*]
            [$command]
            [cluster $cluster_id]
            [$cluster_variant]
            [$try_module::$try_variant_or_module $(::$try_variant)?]
            [$command_id]
            [$direction]
            [$(const DISABLE_DEFAULT_RESPONSE: bool = $disable_default_response;)?]
            [$($response)?]
            [$($($extra_derive),*)?]
            []
            $($rest)*
        }
    };
    (
        $(#[$attr:meta])*
        $command:ident {
            { $cluster_id:expr } => $cluster_variant:ident;
            command_id: $command_id:expr;
            direction: $direction:expr;
            $(disable_default_response: $disable_default_response:expr;)?
            $(response: $response:ty;)?
            => $try_module:ident::$try_variant_or_module:ident $(::$try_variant:ident)?;
            $(derive($($extra_derive:path),* $(,)?);)?
            fields {
                $(
                    $(#[$field_attr:meta])*
                    $field:ident: $field_ty:ty
                ),* $(,)?
            }
            $($rest:tt)*
        }
    ) => {
        $crate::macros::zcl_command! {
            @parse_constructor
            [named]
            [$(#[$attr])*]
            [$command]
            [cluster $cluster_id]
            [$cluster_variant]
            [$try_module::$try_variant_or_module $(::$try_variant)?]
            [$command_id]
            [$direction]
            [$(const DISABLE_DEFAULT_RESPONSE: bool = $disable_default_response;)?]
            [$($response)?]
            [$($($extra_derive),*)?]
            [$($(#[$field_attr])* $field: $field_ty,)*]
            $($rest)*
        }
    };
    (
        $(#[$attr:meta])*
        $command:ident {
            $cluster_variant:ident;
            command_id: $command_id:expr;
            direction: $direction:expr;
            $(disable_default_response: $disable_default_response:expr;)?
            $(response: $response:ty;)?
            => $try_module:ident::$try_variant_or_module:ident $(::$try_variant:ident)?;
            $(derive($($extra_derive:path),* $(,)?);)?
            fields;
            $($rest:tt)*
        }
    ) => {
        $crate::macros::zcl_command! {
            @parse_constructor
            [unit]
            [$(#[$attr])*]
            [$command]
            [global]
            [$cluster_variant]
            [$try_module::$try_variant_or_module $(::$try_variant)?]
            [$command_id]
            [$direction]
            [$(const DISABLE_DEFAULT_RESPONSE: bool = $disable_default_response;)?]
            [$($response)?]
            [$($($extra_derive),*)?]
            []
            $($rest)*
        }
    };
    (
        $(#[$attr:meta])*
        $command:ident {
            $cluster_variant:ident;
            command_id: $command_id:expr;
            direction: $direction:expr;
            $(disable_default_response: $disable_default_response:expr;)?
            $(response: $response:ty;)?
            => $try_module:ident::$try_variant_or_module:ident $(::$try_variant:ident)?;
            $(derive($($extra_derive:path),* $(,)?);)?
            fields {
                $(
                    $(#[$field_attr:meta])*
                    $field:ident: $field_ty:ty
                ),* $(,)?
            }
            $($rest:tt)*
        }
    ) => {
        $crate::macros::zcl_command! {
            @parse_constructor
            [named]
            [$(#[$attr])*]
            [$command]
            [global]
            [$cluster_variant]
            [$try_module::$try_variant_or_module $(::$try_variant)?]
            [$command_id]
            [$direction]
            [$(const DISABLE_DEFAULT_RESPONSE: bool = $disable_default_response;)?]
            [$($response)?]
            [$($($extra_derive),*)?]
            [$($(#[$field_attr])* $field: $field_ty,)*]
            $($rest)*
        }
    };
    (
        @parse_constructor
        $kind:tt
        $attrs:tt
        $command:tt
        $scope:tt
        $cluster_variant:tt
        $try_from:tt
        $command_id:tt
        $direction:tt
        $disable_default_response:tt
        $response:tt
        $extra_derives:tt
        $fields:tt
        constructor { $($constructor:item)* }
        $($rest:tt)*
    ) => {
        $crate::macros::zcl_command! {
            @parse_getters
            $kind
            $attrs
            $command
            $scope
            $cluster_variant
            $try_from
            $command_id
            $direction
            $disable_default_response
            $response
            $extra_derives
            $fields
            [constructor { $($constructor)* }]
            $($rest)*
        }
    };
    (
        @parse_constructor
        $kind:tt
        $attrs:tt
        $command:tt
        $scope:tt
        $cluster_variant:tt
        $try_from:tt
        $command_id:tt
        $direction:tt
        $disable_default_response:tt
        $response:tt
        $extra_derives:tt
        $fields:tt
        $($rest:tt)*
    ) => {
        $crate::macros::zcl_command! {
            @parse_getters
            $kind
            $attrs
            $command
            $scope
            $cluster_variant
            $try_from
            $command_id
            $direction
            $disable_default_response
            $response
            $extra_derives
            $fields
            [default]
            $($rest)*
        }
    };
    (
        @parse_getters
        $kind:tt
        $attrs:tt
        $command:tt
        $scope:tt
        $cluster_variant:tt
        $try_from:tt
        $command_id:tt
        $direction:tt
        $disable_default_response:tt
        $response:tt
        $extra_derives:tt
        $fields:tt
        $constructor:tt
        getters { $($getter:item)* }
        $($rest:tt)*
    ) => {
        $crate::macros::zcl_command! {
            @parse_streams
            $kind
            $attrs
            $command
            $scope
            $cluster_variant
            $try_from
            $command_id
            $direction
            $disable_default_response
            $response
            $extra_derives
            $fields
            $constructor
            [getters { $($getter)* }]
            $($rest)*
        }
    };
    (
        @parse_getters
        $kind:tt
        $attrs:tt
        $command:tt
        $scope:tt
        $cluster_variant:tt
        $try_from:tt
        $command_id:tt
        $direction:tt
        $disable_default_response:tt
        $response:tt
        $extra_derives:tt
        $fields:tt
        $constructor:tt
        $($rest:tt)*
    ) => {
        $crate::macros::zcl_command! {
            @parse_streams
            $kind
            $attrs
            $command
            $scope
            $cluster_variant
            $try_from
            $command_id
            $direction
            $disable_default_response
            $response
            $extra_derives
            $fields
            $constructor
            []
            $($rest)*
        }
    };
    (
        @parse_streams
        $kind:tt
        $attrs:tt
        $command:tt
        $scope:tt
        $cluster_variant:tt
        $try_from:tt
        $command_id:tt
        $direction:tt
        $disable_default_response:tt
        $response:tt
        $extra_derives:tt
        $fields:tt
        $constructor:tt
        $getters:tt
        from_le_stream { $($from_le_stream:item)* }
        to_le_stream { $($to_le_stream:item)* }
        $(impl { $($custom:item)* })?
    ) => {
        $crate::macros::zcl_command! {
            @define
            $kind
            []
            $attrs
            $command
            $scope
            $cluster_variant
            $try_from
            $command_id
            $direction
            $disable_default_response
            $response
            $extra_derives
            $fields
            $constructor
            $getters
            [from_le_stream { $($from_le_stream)* }]
            [to_le_stream { $($to_le_stream)* }]
            [$($($custom)*)?]
        }
    };
    (
        @parse_streams
        $kind:tt
        $attrs:tt
        $command:tt
        $scope:tt
        $cluster_variant:tt
        $try_from:tt
        $command_id:tt
        $direction:tt
        $disable_default_response:tt
        $response:tt
        $extra_derives:tt
        $fields:tt
        $constructor:tt
        $getters:tt
        from_le_stream { $($from_le_stream:item)* }
        $(impl { $($custom:item)* })?
    ) => {
        $crate::macros::zcl_command! {
            @define
            $kind
            [le_stream::ToLeStream]
            $attrs
            $command
            $scope
            $cluster_variant
            $try_from
            $command_id
            $direction
            $disable_default_response
            $response
            $extra_derives
            $fields
            $constructor
            $getters
            [from_le_stream { $($from_le_stream)* }]
            []
            [$($($custom)*)?]
        }
    };
    (
        @parse_streams
        $kind:tt
        $attrs:tt
        $command:tt
        $scope:tt
        $cluster_variant:tt
        $try_from:tt
        $command_id:tt
        $direction:tt
        $disable_default_response:tt
        $response:tt
        $extra_derives:tt
        $fields:tt
        $constructor:tt
        $getters:tt
        to_le_stream { $($to_le_stream:item)* }
        $(impl { $($custom:item)* })?
    ) => {
        $crate::macros::zcl_command! {
            @define
            $kind
            [le_stream::FromLeStream]
            $attrs
            $command
            $scope
            $cluster_variant
            $try_from
            $command_id
            $direction
            $disable_default_response
            $response
            $extra_derives
            $fields
            $constructor
            $getters
            []
            [to_le_stream { $($to_le_stream)* }]
            [$($($custom)*)?]
        }
    };
    (
        @parse_streams
        $kind:tt
        $attrs:tt
        $command:tt
        $scope:tt
        $cluster_variant:tt
        $try_from:tt
        $command_id:tt
        $direction:tt
        $disable_default_response:tt
        $response:tt
        $extra_derives:tt
        $fields:tt
        $constructor:tt
        $getters:tt
        $(impl { $($custom:item)* })?
    ) => {
        $crate::macros::zcl_command! {
            @define
            $kind
            [le_stream::FromLeStream, le_stream::ToLeStream]
            $attrs
            $command
            $scope
            $cluster_variant
            $try_from
            $command_id
            $direction
            $disable_default_response
            $response
            $extra_derives
            $fields
            $constructor
            $getters
            []
            []
            [$($($custom)*)?]
        }
    };
    (
        @define
        [unit]
        [$($stream_derive:path),*]
        [$(#[$attr:meta])*]
        [$command:ident]
        $scope:tt
        [$cluster_variant:ident]
        $try_from:tt
        [$command_id:expr]
        [$direction:expr]
        [$($disable_default_response:tt)*]
        [$($response:tt)*]
        [$($extra_derive:path),*]
        []
        $constructor:tt
        $getters:tt
        $from_le_stream:tt
        $to_le_stream:tt
        [$($custom:item)*]
    ) => {
        $(#[$attr])*
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd
            $(, $stream_derive)*
            $(, $extra_derive)*
        )]
        pub struct $command;

        $crate::macros::zcl_command! {
            @constructor_impl
            $command
            $constructor
            []
        }

        $crate::macros::zcl_command! {
            @getters_impl
            $command
            $getters
        }

        $crate::macros::zcl_command! {
            @impls
            $command
            $scope
            [$cluster_variant]
            $try_from
            [$command_id]
            [$direction]
            [$($disable_default_response)*]
            [$($response)*]
            $from_le_stream
            $to_le_stream
            [$($custom)*]
        }
    };
    (
        @define
        [named]
        [$($stream_derive:path),*]
        [$(#[$attr:meta])*]
        [$command:ident]
        $scope:tt
        [$cluster_variant:ident]
        $try_from:tt
        [$command_id:expr]
        [$direction:expr]
        [$($disable_default_response:tt)*]
        [$($response:tt)*]
        [$($extra_derive:path),*]
        [$($(#[$field_attr:meta])* $field:ident: $field_ty:ty,)*]
        $constructor:tt
        $getters:tt
        $from_le_stream:tt
        $to_le_stream:tt
        [$($custom:item)*]
    ) => {
        $(#[$attr])*
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Debug, Eq, PartialEq, Hash $(, $stream_derive)* $(, $extra_derive)*)]
        pub struct $command {
            $(
                $(#[$field_attr])*
                $field: $field_ty,
            )*
        }

        $crate::macros::zcl_command! {
            @constructor_impl
            $command
            $constructor
            [$($(#[$field_attr])* $field: $field_ty,)*]
        }

        $crate::macros::zcl_command! {
            @getters_impl
            $command
            $getters
        }

        $crate::macros::zcl_command! {
            @impls
            $command
            $scope
            [$cluster_variant]
            $try_from
            [$command_id]
            [$direction]
            [$($disable_default_response)*]
            [$($response)*]
            $from_le_stream
            $to_le_stream
            [$($custom)*]
        }
    };
    (
        @constructor_impl
        $command:ident
        [constructor { $($constructor:item)* }]
        $fields:tt
    ) => {
        impl $command {
            $($constructor)*
        }
    };
    (
        @constructor_impl
        $command:ident
        [default]
        []
    ) => {
        impl $command {
            /// Creates a new command payload.
            #[allow(clippy::new_without_default)]
            #[must_use]
            pub const fn new() -> Self {
                Self
            }
        }
    };
    (
        @constructor_impl
        $command:ident
        [default]
        [$($(#[$field_attr:meta])* $field:ident: $field_ty:ty,)*]
    ) => {
        impl $command {
            /// Creates a new command payload.
            #[must_use]
            pub const fn new($($field: $field_ty),*) -> Self {
                Self { $($field,)* }
            }
        }
    };
    (@getters_impl $command:ident []) => {};
    (
        @getters_impl
        $command:ident
        [getters { $($getter:item)* }]
    ) => {
        impl $command {
            $($getter)*
        }
    };
    (
        @impls
        $command:ident
        [cluster $cluster_id:expr]
        [$cluster_variant:ident]
        $try_from:tt
        [$command_id:expr]
        [$direction:expr]
        [$($disable_default_response:tt)*]
        [$($response:tt)*]
        $from_le_stream:tt
        $to_le_stream:tt
        [$($custom:item)*]
    ) => {
        impl apis_saltans_core::Cluster<apis_saltans_core::ClusterId> for $command {
            const ID: apis_saltans_core::ClusterId = $cluster_id;
        }

        $crate::macros::zcl_command! {
            @command_impls
            $command
            [$cluster_variant]
            $try_from
            [$command_id]
            [$direction]
            [$($disable_default_response)*]
            [$($response)*]
            $from_le_stream
            $to_le_stream
            [$($custom)*]
        }
    };
    (
        @impls
        $command:ident
        [global]
        [$cluster_variant:ident]
        $try_from:tt
        [$command_id:expr]
        [$direction:expr]
        [$($disable_default_response:tt)*]
        [$($response:tt)*]
        $from_le_stream:tt
        $to_le_stream:tt
        [$($custom:item)*]
    ) => {
        impl $crate::command::Scoped for $command {
            const SCOPE: $crate::Scope = $crate::Scope::Global;
        }

        $crate::macros::zcl_command! {
            @command_impls
            $command
            [$cluster_variant]
            $try_from
            [$command_id]
            [$direction]
            [$($disable_default_response)*]
            [$($response)*]
            $from_le_stream
            $to_le_stream
            [$($custom)*]
        }
    };
    (
        @command_impls
        $command:ident
        [$cluster_variant:ident]
        $try_from:tt
        [$command_id:expr]
        [$direction:expr]
        [$($disable_default_response:tt)*]
        [$($response:tt)*]
        [$(from_le_stream { $($from_le_stream:item)* })?]
        [$(to_le_stream { $($to_le_stream:item)* })?]
        [$($custom:item)*]
    ) => {
        impl $crate::Command for $command {
            const ID: u8 = $command_id;
            const DIRECTION: $crate::Direction = $direction;
            $($disable_default_response)*
        }

        $crate::macros::zcl_command! {
            @response_impl
            $command
            [$($response)*]
        }

        $crate::macros::zcl_command! {
            @from_cluster
            $command
            [$cluster_variant]
            $try_from
        }

        $crate::macros::zcl_command! {
            @try_from_cluster
            $command
            [$cluster_variant]
            $try_from
        }

        $(
            impl le_stream::FromLeStream for $command {
                $($from_le_stream)*
            }
        )?

        $(
            impl le_stream::ToLeStream for $command {
                $($to_le_stream)*
            }
        )?

        $($custom)*
    };
    (@response_impl $command:ident []) => {};
    (@response_impl $command:ident [$response:ty]) => {
        impl apis_saltans_core::ExpectResponse<$crate::Cluster> for $command {
            type Response = $response;
        }
    };
    (
        @from_cluster
        $command:ident
        [$cluster_variant:ident]
        [$try_module:ident::$try_variant:ident]
    ) => {
        impl From<$command> for $crate::Cluster {
            fn from(command: $command) -> Self {
                Self::$cluster_variant($try_module::Command::$try_variant(command.into()))
            }
        }
    };
    (
        @from_cluster
        $command:ident
        [$cluster_variant:ident]
        [$try_module:ident::$try_submodule:ident::$try_variant:ident]
    ) => {
        impl From<$command> for $crate::Cluster {
            fn from(command: $command) -> Self {
                Self::$cluster_variant(
                    $try_module::$try_submodule::Command::$try_variant(command.into()),
                )
            }
        }
    };
    (
        @try_from_cluster
        $command:ident
        [$cluster_variant:ident]
        [$try_module:ident::$try_variant:ident]
    ) => {
        impl TryFrom<$crate::Cluster> for $command {
            type Error = $crate::Cluster;

            fn try_from(cluster: $crate::Cluster) -> Result<Self, Self::Error> {
                if let $crate::Cluster::$cluster_variant($try_module::Command::$try_variant(command)) =
                    cluster
                {
                    Ok(*command)
                } else {
                    Err(cluster)
                }
            }
        }
    };
    (
        @try_from_cluster
        $command:ident
        [$cluster_variant:ident]
        [$try_module:ident::$try_submodule:ident::$try_variant:ident]
    ) => {
        impl TryFrom<$crate::Cluster> for $command {
            type Error = $crate::Cluster;

            fn try_from(cluster: $crate::Cluster) -> Result<Self, Self::Error> {
                if let $crate::Cluster::$cluster_variant(
                    $try_module::$try_submodule::Command::$try_variant(command),
                ) = cluster
                {
                    Ok(*command)
                } else {
                    Err(cluster)
                }
            }
        }
    };
}

pub(crate) use zcl_command;

macro_rules! zcl_command_enum {
    (
        $(#[$attr:meta])*
        { $cluster_id:expr } => $cluster_name:ident;
        $($command:ident),+ $(,)?
    ) => {
        $crate::macros::zcl_command_enum! {
            @define
            [$(#[$attr])*]
            [$cluster_name]
            [cluster $cluster_id]
            [$($command($command)),+]
        }
    };
    (
        $(#[$attr:meta])*
        $cluster_name:ident;
        $($command:ident),+ $(,)?
    ) => {
        $crate::macros::zcl_command_enum! {
            @define
            [$(#[$attr])*]
            [$cluster_name]
            [global]
            [$($command($command)),+]
        }
    };
    (
        $(#[$attr:meta])*
        { $cluster_id:expr } => $cluster_name:ident;
        $($variant:ident($command:ty)),+ $(,)?
    ) => {
        $crate::macros::zcl_command_enum! {
            @define
            [$(#[$attr])*]
            [$cluster_name]
            [cluster $cluster_id]
            [$($variant($command)),+]
        }
    };
    (
        $(#[$attr:meta])*
        $cluster_name:ident;
        $($variant:ident($command:ty)),+ $(,)?
    ) => {
        $crate::macros::zcl_command_enum! {
            @define
            [$(#[$attr])*]
            [$cluster_name]
            [global]
            [$($variant($command)),+]
        }
    };
    (
        @define
        [$(#[$attr:meta])*]
        [$cluster_name:ident]
        $scope:tt
        [$($variant:ident($command:ty)),+]
    ) => {
        $(#[$attr])*
        /// Available ZCL commands.
        #[derive(Clone, Debug, Eq, PartialEq, Hash, apis_saltans_macros::ParseZclFrame)]
        pub enum Command {
            $(
                /// ZCL command variant.
                $variant(std::boxed::Box<$command>),
            )+
        }

        $crate::macros::zcl_command_enum! {
            @cluster_impl
            $scope
        }

        impl From<Command> for $crate::Cluster {
            fn from(command: Command) -> Self {
                Self::$cluster_name(command)
            }
        }

        $(
            impl From<$command> for Command {
                fn from(command: $command) -> Self {
                    Self::$variant(command.into())
                }
            }
        )+

        impl $crate::CommandDispatch for Command {
            fn command_id(&self) -> u8 {
                match self {
                    $(
                        Self::$variant(command) => {
                            $crate::CommandDispatch::command_id(command.as_ref())
                        }
                    )+
                }
            }

            fn scope(&self) -> $crate::Scope {
                match self {
                    $(
                        Self::$variant(command) => {
                            $crate::CommandDispatch::scope(command.as_ref())
                        }
                    )+
                }
            }

            fn direction(&self) -> apis_saltans_core::Direction {
                match self {
                    $(
                        Self::$variant(command) => {
                            $crate::CommandDispatch::direction(command.as_ref())
                        }
                    )+
                }
            }

            fn disable_default_response(&self) -> bool {
                match self {
                    $(
                        Self::$variant(command) => {
                            $crate::CommandDispatch::disable_default_response(command.as_ref())
                        }
                    )+
                }
            }
        }

        impl le_stream::ToLeStream for Command {
            type Iter = iterator::Iter;

            fn to_le_stream(self) -> Self::Iter {
                match self {
                    $(
                        Self::$variant(command) => {
                            iterator::Iter::$variant(
                                le_stream::ToLeStream::to_le_stream(*command).into(),
                            )
                        }
                    )+
                }
            }
        }

        mod iterator {
            use super::*;

            pub enum Iter {
                $(
                    $variant(std::boxed::Box<<$command as le_stream::ToLeStream>::Iter>),
                )+
            }

            impl Iterator for Iter {
                type Item = u8;

                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        $(
                            Self::$variant(iter) => iter.next(),
                        )+
                    }
                }
            }
        }
    };
    (@cluster_impl [global]) => {};
    (@cluster_impl [cluster $cluster_id:expr]) => {
        impl apis_saltans_core::Cluster<apis_saltans_core::ClusterId> for Command {
            const ID: apis_saltans_core::ClusterId = $cluster_id;
        }
    };
}

pub(crate) use zcl_command_enum;
