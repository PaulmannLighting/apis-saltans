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
///         fields;
///     }
/// }
/// ```
///
/// Global commands omit the cluster ID and implement [`crate::command::Scoped`]
/// with [`crate::Scope::Global`] instead. If the global command enum variant
/// differs from the generated type name, pass that variant explicitly. A
/// `constructor` section can override the generated `new` constructor, and a
/// `getters` section can contain accessor methods. Optional `from_le_stream`
/// and `to_le_stream` sections replace the respective derive, and the final
/// `impl` section can contain custom inherent or trait implementations for the
/// type:
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
            [super::$command]
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
            [super::$command]
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
        $(#[$attr:meta])*
        $command:ident {
            $cluster_variant:ident;
            command_id: $command_id:expr;
            direction: $direction:expr;
            $(disable_default_response: $disable_default_response:expr;)?
            $(response: $response:ty;)?
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
            [crate::global::$command]
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
            [crate::global::$command]
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
        #[derive(
            Clone,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd
            $(, $stream_derive)*
            $(, $extra_derive)*
        )]
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

/// Define access-specific ZCL attribute enums from one attribute table.
///
/// The macro generates fixed enum names in the invocation module:
/// `Id` for readable attribute IDs, plus `Readable`, `Writable`,
/// `Reportable`, and `Scene` for access-specific attribute values.
///
/// ```ignore
/// zcl_attributes! {
///     cluster: ClusterId::OnOff;
///     manufacturer_code: 0x1234;
///
///     /// On/Off state.
///     OnOff = 0x0000: Bool { R, W, P, S },
///     /// Start-up behavior.
///     StartUpOnOff = 0x4003: StartUpOnOff { R, W },
/// }
/// ```
#[allow(unused_macros)]
macro_rules! zcl_attributes {
    (
        cluster: $cluster_id:expr;
        manufacturer_code: $manufacturer_code:expr;
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ty {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            [cluster $cluster_id]
            [$manufacturer_code]
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty {
                    $($access)*
                }
            ),*
        }
    };
    (
        cluster: $cluster_id:expr;
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ty {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            [cluster $cluster_id]
            []
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty {
                    $($access)*
                }
            ),*
        }
    };
    (
        { $cluster_id:expr };
        manufacturer_code: $manufacturer_code:expr;
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ty {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            [cluster $cluster_id]
            [$manufacturer_code]
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty {
                    $($access)*
                }
            ),*
        }
    };
    (
        { $cluster_id:expr };
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ty {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            [cluster $cluster_id]
            []
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty {
                    $($access)*
                }
            ),*
        }
    };
    (
        manufacturer_code: $manufacturer_code:expr;
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ty {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            []
            [$manufacturer_code]
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty {
                    $($access)*
                }
            ),*
        }
    };
    (
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ty {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            []
            []
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty {
                    $($access)*
                }
            ),*
        }
    };
    (
        @define
        $cluster:tt
        [$($manufacturer_code:expr)?]
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ty {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define_readable
            $cluster
            [$($manufacturer_code)?]
            [] [] [] [] [] []
            [$([$(#[$variant_attr])*] [$variant] [$id] [$ty] [$($access)*];)*]
        }

        $crate::macros::zcl_attributes! {
            @define_writable
            $cluster
            [$($manufacturer_code)?]
            [] [] []
            [$([$(#[$variant_attr])*] [$variant] [$id] [$ty] [$($access)*];)*]
        }

        $crate::macros::zcl_attributes! {
            @define_data_enum
            [Reportable]
            ["Attributes that can be reported."]
            [P]
            []
            [$([$(#[$variant_attr])*] [$variant] [$id] [$ty] [$($access)*];)*]
        }

        $crate::macros::zcl_attributes! {
            @define_data_enum
            [Scene]
            ["Attributes that can be stored in scenes."]
            [S]
            []
            [$([$(#[$variant_attr])*] [$variant] [$id] [$ty] [$($access)*];)*]
        }
    };
    (@manufacturer_code []) => {};
    (@manufacturer_code [$manufacturer_code:expr]) => {
        const MANUFACTURER_CODE: Option<u16> = Some($manufacturer_code);
    };
    (@cluster_impl [] $ty:ident) => {};
    (@cluster_impl [cluster $cluster_id:expr] $ty:ident) => {
        impl apis_saltans_core::Cluster<apis_saltans_core::ClusterId> for $ty {
            const ID: apis_saltans_core::ClusterId = $cluster_id;
        }
    };
    (@readable_attribute_impl [] [$($manufacturer_code:expr)?]) => {
        impl $crate::ReadableAttribute for Id {
            type Attribute = Readable;

            $crate::macros::zcl_attributes! {
                @manufacturer_code [$($manufacturer_code)?]
            }
        }
    };
    (@readable_attribute_impl [cluster $cluster_id:expr] [$($manufacturer_code:expr)?]) => {
        impl $crate::ReadableAttribute for Id {
            type Attribute = Readable;

            $crate::macros::zcl_attributes! {
                @manufacturer_code [$($manufacturer_code)?]
            }
        }
    };
    (@writable_attribute_impl [] [$($manufacturer_code:expr)?] [$($id_arms:tt)*]) => {
        impl $crate::WritableAttribute for Writable {
            $crate::macros::zcl_attributes! {
                @manufacturer_code [$($manufacturer_code)?]
            }

            fn id(&self) -> u16 {
                match self {
                    $($id_arms)*
                }
            }
        }
    };
    (
        @writable_attribute_impl
        [cluster $cluster_id:expr]
        [$($manufacturer_code:expr)?]
        [$($id_arms:tt)*]
    ) => {
        impl $crate::WritableAttribute for Writable {
            $crate::macros::zcl_attributes! {
                @manufacturer_code [$($manufacturer_code)?]
            }

            fn id(&self) -> u16 {
                match self {
                    $($id_arms)*
                }
            }
        }
    };
    (
        @define_readable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($id_variants:tt)*]
        [$($readable_variants:tt)*]
        [$($from_id_arms:tt)*]
        [$($try_from_u16_arms:tt)*]
        [$($try_from_readable_arms:tt)*]
        [$($from_readable_arms:tt)*]
        []
    ) => {
        $crate::macros::zcl_attributes! {
            @emit_id_enum
            [$($id_variants)*]
        }

        $crate::macros::zcl_attributes! {
            @emit_value_enum
            [Readable]
            ["Attributes that can be read."]
            [$($readable_variants)*]
        }

        $crate::macros::zcl_attributes! {
            @cluster_impl
            $cluster
            Id
        }

        $crate::macros::zcl_attributes! {
            @readable_attribute_impl
            $cluster
            [$($manufacturer_code)?]
        }

        impl From<Id> for u16 {
            fn from(id: Id) -> Self {
                match id {
                    $($from_id_arms)*
                }
            }
        }

        impl TryFrom<u16> for Id {
            type Error = u16;

            fn try_from(id: u16) -> Result<Self, Self::Error> {
                match id {
                    $($try_from_u16_arms)*
                    other => Err(other),
                }
            }
        }

        impl core::fmt::Display for Id {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{self:?}")
            }
        }

        impl TryFrom<(Id, apis_saltans_core::types::Type)> for Readable {
            type Error = $crate::InvalidType<Id>;

            fn try_from(
                (id, typ): (Id, apis_saltans_core::types::Type),
            ) -> Result<Self, Self::Error> {
                match (id, typ) {
                    $($try_from_readable_arms)*
                }
                .map_err(|typ| $crate::InvalidType::new(id, typ))
            }
        }

        impl From<Readable> for apis_saltans_core::types::Type {
            fn from(attribute: Readable) -> Self {
                match attribute {
                    $($from_readable_arms)*
                }
            }
        }
    };
    (
        @define_readable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($id_variants:tt)*]
        [$($readable_variants:tt)*]
        [$($from_id_arms:tt)*]
        [$($try_from_u16_arms:tt)*]
        [$($try_from_readable_arms:tt)*]
        [$($from_readable_arms:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [$($access:tt)*]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @readable_access
            $cluster
            [$($manufacturer_code)?]
            [$($id_variants)*]
            [$($readable_variants)*]
            [$($from_id_arms)*]
            [$($try_from_u16_arms)*]
            [$($try_from_readable_arms)*]
            [$($from_readable_arms)*]
            [$($rest)*]
            [$($variant_attr)*]
            [$variant]
            [$id]
            [$ty]
            [$($access)*]
        }
    };
    (
        @readable_access
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($id_variants:tt)*]
        [$($readable_variants:tt)*]
        [$($from_id_arms:tt)*]
        [$($try_from_u16_arms:tt)*]
        [$($try_from_readable_arms:tt)*]
        [$($from_readable_arms:tt)*]
        [$($rest:tt)*]
        [$($variant_attr:tt)*]
        [$variant:ident]
        [$id:tt]
        [$ty:ty]
        []
    ) => {
        $crate::macros::zcl_attributes! {
            @define_readable
            $cluster
            [$($manufacturer_code)?]
            [$($id_variants)*]
            [$($readable_variants)*]
            [$($from_id_arms)*]
            [$($try_from_u16_arms)*]
            [$($try_from_readable_arms)*]
            [$($from_readable_arms)*]
            [$($rest)*]
        }
    };
    (
        @readable_access
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($id_variants:tt)*]
        [$($readable_variants:tt)*]
        [$($from_id_arms:tt)*]
        [$($try_from_u16_arms:tt)*]
        [$($try_from_readable_arms:tt)*]
        [$($from_readable_arms:tt)*]
        [$($rest:tt)*]
        [$($variant_attr:tt)*]
        [$variant:ident]
        [$id:tt]
        [$ty:ty]
        [R $(, $($tail:tt)*)?]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_readable
            $cluster
            [$($manufacturer_code)?]
            [$($id_variants)* $($variant_attr)* $variant = $id,]
            [$($readable_variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($from_id_arms)* Id::$variant => $id,]
            [$($try_from_u16_arms)* $id => Ok(Id::$variant),]
            [$($try_from_readable_arms)* (Id::$variant, typ) => typ.try_into().map(Readable::$variant),]
            [$($from_readable_arms)* Readable::$variant(value) => value.into(),]
            [$($rest)*]
        }
    };
    (
        @readable_access
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($id_variants:tt)*]
        [$($readable_variants:tt)*]
        [$($from_id_arms:tt)*]
        [$($try_from_u16_arms:tt)*]
        [$($try_from_readable_arms:tt)*]
        [$($from_readable_arms:tt)*]
        [$($rest:tt)*]
        [$($variant_attr:tt)*]
        [$variant:ident]
        [$id:tt]
        [$ty:ty]
        [$ignored:tt $(, $($tail:tt)*)?]
    ) => {
        $crate::macros::zcl_attributes! {
            @readable_access
            $cluster
            [$($manufacturer_code)?]
            [$($id_variants)*]
            [$($readable_variants)*]
            [$($from_id_arms)*]
            [$($try_from_u16_arms)*]
            [$($try_from_readable_arms)*]
            [$($from_readable_arms)*]
            [$($rest)*]
            [$($variant_attr)*]
            [$variant]
            [$id]
            [$ty]
            [$($($tail)*)?]
        }
    };
    (
        @define_writable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($writable_variants:tt)*]
        [$($id_arms:tt)*]
        [$($record_arms:tt)*]
        []
    ) => {
        $crate::macros::zcl_attributes! {
            @emit_value_enum
            [Writable]
            ["Attributes that can be written."]
            [$($writable_variants)*]
        }

        $crate::macros::zcl_attributes! {
            @cluster_impl
            $cluster
            Writable
        }

        $crate::macros::zcl_attributes! {
            @writable_attribute_impl
            $cluster
            [$($manufacturer_code)?]
            [$($id_arms)*]
        }

        impl From<Writable> for $crate::global::write_attributes::Record {
            fn from(attribute: Writable) -> Self {
                match attribute {
                    $($record_arms)*
                }
            }
        }
    };
    (
        @define_writable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($writable_variants:tt)*]
        [$($id_arms:tt)*]
        [$($record_arms:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [$($access:tt)*]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @writable_access
            $cluster
            [$($manufacturer_code)?]
            [$($writable_variants)*]
            [$($id_arms)*]
            [$($record_arms)*]
            [$($rest)*]
            [$($variant_attr)*]
            [$variant]
            [$id]
            [$ty]
            [$($access)*]
        }
    };
    (
        @writable_access
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($writable_variants:tt)*]
        [$($id_arms:tt)*]
        [$($record_arms:tt)*]
        [$($rest:tt)*]
        [$($variant_attr:tt)*]
        [$variant:ident]
        [$id:tt]
        [$ty:ty]
        []
    ) => {
        $crate::macros::zcl_attributes! {
            @define_writable
            $cluster
            [$($manufacturer_code)?]
            [$($writable_variants)*]
            [$($id_arms)*]
            [$($record_arms)*]
            [$($rest)*]
        }
    };
    (
        @writable_access
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($writable_variants:tt)*]
        [$($id_arms:tt)*]
        [$($record_arms:tt)*]
        [$($rest:tt)*]
        [$($variant_attr:tt)*]
        [$variant:ident]
        [$id:tt]
        [$ty:ty]
        [W $(, $($tail:tt)*)?]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_writable
            $cluster
            [$($manufacturer_code)?]
            [$($writable_variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($id_arms)* Writable::$variant(_) => $id,]
            [$($record_arms)* Writable::$variant(value) => $crate::global::write_attributes::Record::new($id, value.into()),]
            [$($rest)*]
        }
    };
    (
        @writable_access
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($writable_variants:tt)*]
        [$($id_arms:tt)*]
        [$($record_arms:tt)*]
        [$($rest:tt)*]
        [$($variant_attr:tt)*]
        [$variant:ident]
        [$id:tt]
        [$ty:ty]
        [$ignored:tt $(, $($tail:tt)*)?]
    ) => {
        $crate::macros::zcl_attributes! {
            @writable_access
            $cluster
            [$($manufacturer_code)?]
            [$($writable_variants)*]
            [$($id_arms)*]
            [$($record_arms)*]
            [$($rest)*]
            [$($variant_attr)*]
            [$variant]
            [$id]
            [$ty]
            [$($($tail)*)?]
        }
    };
    (
        @define_data_enum
        [$enum:ident]
        [$doc:literal]
        [$access:tt]
        [$($variants:tt)*]
        []
    ) => {
        $crate::macros::zcl_attributes! { @emit_value_enum [$enum] [$doc] [$($variants)*] }
    };
    (
        @define_data_enum
        [$enum:ident]
        [$doc:literal]
        [$access:tt]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [$($flags:tt)*]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @data_access
            [$enum]
            [$doc]
            [$access]
            [$($variants)*]
            [$($rest)*]
            [$($variant_attr)*]
            [$variant]
            [$id]
            [$ty]
            [$($flags)*]
        }
    };
    (@data_access [$enum:ident] [$doc:literal] [$access:tt] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] []) => {
        $crate::macros::zcl_attributes! { @define_data_enum [$enum] [$doc] [$access] [$($variants)*] [$($rest)*] }
    };
    (@data_access [Reportable] [$doc:literal] [P] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [P $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @define_data_enum [Reportable] [$doc] [P] [$($variants)* $($variant_attr)* $variant($ty) = $id,] [$($rest)*] }
    };
    (@data_access [Scene] [$doc:literal] [S] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [S $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @define_data_enum [Scene] [$doc] [S] [$($variants)* $($variant_attr)* $variant($ty) = $id,] [$($rest)*] }
    };
    (@data_access [$enum:ident] [$doc:literal] [$access:tt] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [$ignored:tt $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @data_access [$enum] [$doc] [$access] [$($variants)*] [$($rest)*] [$($variant_attr)*] [$variant] [$id] [$ty] [$($($tail)*)?] }
    };
    (@emit_id_enum []) => {
        /// IDs of readable attributes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Id {}
    };
    (@emit_id_enum [$($variants:tt)+]) => {
        /// IDs of readable attributes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(u16)]
        pub enum Id {
            $($variants)+
        }
    };
    (@emit_value_enum [$enum:ident] [$doc:literal] []) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub enum $enum {}
    };
    (@emit_value_enum [$enum:ident] [$doc:literal] [$($variants:tt)+]) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        #[repr(u16)]
        pub enum $enum {
            $($variants)+
        }
    };
}

#[allow(unused_imports)]
pub(crate) use zcl_attributes;

#[cfg(test)]
mod zcl_attributes_macro_tests {
    use apis_saltans_core::ClusterId;
    use apis_saltans_core::types::{Type, Uint8};

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Custom(Uint8);

    impl From<Custom> for Type {
        fn from(value: Custom) -> Self {
            value.0.into()
        }
    }

    zcl_attributes! {
        cluster: ClusterId::OnOff;
        manufacturer_code: 0x1234;

        /// Read-only test attribute.
        ReadOnly = 0x0000: Uint8 { R },
        /// Writable test attribute.
        Writable = 0x0001: Uint8 { R, W, P, S },
        /// Write-only test attribute.
        WriteOnly = 0x0002: Custom { W },
    }

    #[test]
    fn generates_access_specific_enums() {
        let _ = Id::ReadOnly;
        let _ = Readable::ReadOnly(Uint8::new(1));
        let _ = Readable::Writable(Uint8::new(2));
        let _ = Writable::Writable(Uint8::new(3));
        let _ = Writable::WriteOnly(Custom(Uint8::new(4)));
        let _ = Reportable::Writable(Uint8::new(5));
        let _ = Scene::Writable(Uint8::new(6));
    }

    mod without_cluster {
        use super::Uint8;

        zcl_attributes! {
            /// Read-only test attribute.
            ReadOnly = 0x0000: Uint8 { R },
            /// Writable test attribute.
            Writable = 0x0001: Uint8 { W, P, S },
        }

        #[test]
        fn generates_without_cluster_bound_impls() {
            fn assert_readable<T>()
            where
                T: crate::ReadableAttribute,
            {
            }

            fn assert_writable<T>()
            where
                T: crate::WritableAttribute,
            {
            }

            assert_readable::<Id>();
            assert_writable::<Writable>();

            let _ = Id::ReadOnly;
            let _ = Readable::ReadOnly(Uint8::new(1));
            let _ = Writable::Writable(Uint8::new(2));
            let _ = Reportable::Writable(Uint8::new(3));
            let _ = Scene::Writable(Uint8::new(4));
        }
    }
}
