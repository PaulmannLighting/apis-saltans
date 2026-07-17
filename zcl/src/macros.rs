macro_rules! zcl_cluster_profile {
    ([]) => {
        zb_core::Profile::ZigbeeHomeAutomation
    };
    ([$profile:expr]) => {
        $profile
    };
}

pub(crate) use zcl_cluster_profile;

/// Define a ZCL command payload and implement its common command traits.
///
/// Cluster-specific commands use the cluster ID to implement
/// [`zb_core::ClusterSpecific`]:
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
/// `direction` declaration implements [`crate::Directed`] and makes that
/// direction the default parsing direction. Commands accepted in both
/// directions can omit `direction` and specify `parse_direction` explicitly. A
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
            $(profile: $profile:expr;)?
            command_id: $command_id:expr;
            $(direction: $direction:expr;)?
            $(parse_direction: $parse_direction:expr;)?
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
            [cluster $cluster_id; $($profile)?]
            [$cluster_variant]
            [super::$command]
            [$command_id]
            [$($direction)?]
            [$($parse_direction)?]
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
            $(profile: $profile:expr;)?
            command_id: $command_id:expr;
            $(direction: $direction:expr;)?
            $(parse_direction: $parse_direction:expr;)?
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
            [cluster $cluster_id; $($profile)?]
            [$cluster_variant]
            [super::$command]
            [$command_id]
            [$($direction)?]
            [$($parse_direction)?]
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
            $(direction: $direction:expr;)?
            $(parse_direction: $parse_direction:expr;)?
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
            [$($direction)?]
            [$($parse_direction)?]
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
            $(direction: $direction:expr;)?
            $(parse_direction: $parse_direction:expr;)?
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
            [$($direction)?]
            [$($parse_direction)?]
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
            $(direction: $direction:expr;)?
            $(parse_direction: $parse_direction:expr;)?
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
            [$($direction)?]
            [$($parse_direction)?]
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
            $(direction: $direction:expr;)?
            $(parse_direction: $parse_direction:expr;)?
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
            [$($direction)?]
            [$($parse_direction)?]
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
        $parse_direction:tt
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
            $parse_direction
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
        $parse_direction:tt
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
            $parse_direction
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
        $parse_direction:tt
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
            $parse_direction
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
        $parse_direction:tt
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
            $parse_direction
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
        $parse_direction:tt
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
            $parse_direction
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
        $parse_direction:tt
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
            $parse_direction
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
        $parse_direction:tt
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
            $parse_direction
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
        $parse_direction:tt
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
            $parse_direction
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
        [$($direction:expr)?]
        $parse_direction:tt
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
            [$($direction)?]
            $parse_direction
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
        [$($direction:expr)?]
        $parse_direction:tt
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
            [$($direction)?]
            $parse_direction
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
        [cluster $cluster_id:expr; $($profile:expr)?]
        [$cluster_variant:ident]
        $try_from:tt
        [$command_id:expr]
        [$($direction:expr)?]
        $parse_direction:tt
        [$($disable_default_response:tt)*]
        [$($response:tt)*]
        $from_le_stream:tt
        $to_le_stream:tt
        [$($custom:item)*]
    ) => {
        impl zb_core::ClusterSpecific<zb_core::Cluster> for $command {
            const ID: zb_core::Cluster = $cluster_id;
        }

        impl zb_core::Profiled for $command {
            const PROFILE: zb_core::Profile =
                $crate::macros::zcl_cluster_profile!([$($profile)?]);
        }

        $crate::macros::zcl_command! {
            @command_impls
            $command
            [$cluster_variant]
            $try_from
            [$command_id]
            [$($direction)?]
            $parse_direction
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
        [$($direction:expr)?]
        $parse_direction:tt
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
            [$($direction)?]
            $parse_direction
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
        [$($direction:expr)?]
        $parse_direction:tt
        [$($disable_default_response:tt)*]
        [$($response:tt)*]
        [$(from_le_stream { $($from_le_stream:item)* })?]
        [$(to_le_stream { $($to_le_stream:item)* })?]
        [$($custom:item)*]
    ) => {
        impl $crate::Command for $command {
            const ID: u8 = $command_id;
            $crate::macros::zcl_command! {
                @parse_direction_const
                [$($direction)?]
                $parse_direction
            }
            $($disable_default_response)*
        }

        $crate::macros::zcl_command! {
            @directed_impl
            $command
            [$($direction)?]
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
    (@directed_impl $command:ident []) => {};
    (@directed_impl $command:ident [$direction:expr]) => {
        impl $crate::Directed for $command {
            const DIRECTION: $crate::Direction = $direction;
        }
    };
    (@parse_direction_const [] []) => {};
    (@parse_direction_const [$direction:expr] []) => {
        const PARSE_DIRECTION: $crate::ParseDirection =
            $crate::ParseDirection::Single($direction);
    };
    (@parse_direction_const [] [$parse_direction:expr]) => {
        const PARSE_DIRECTION: $crate::ParseDirection = $parse_direction;
    };
    (@parse_direction_const [$direction:expr] [$parse_direction:expr]) => {
        const PARSE_DIRECTION: $crate::ParseDirection = $parse_direction;
    };
    (@response_impl $command:ident []) => {};
    (@response_impl $command:ident [$response:ty]) => {
        impl zb_core::ExpectResponse<$crate::Cluster> for $command {
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
        $(profile: $profile:expr;)?
        $($command:ident),+ $(,)?
    ) => {
        $crate::macros::zcl_command_enum! {
            @define
            [$(#[$attr])*]
            [$cluster_name]
            [cluster $cluster_id; $($profile)?]
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
        $(profile: $profile:expr;)?
        $($variant:ident($command:ty)),+ $(,)?
    ) => {
        $crate::macros::zcl_command_enum! {
            @define
            [$(#[$attr])*]
            [$cluster_name]
            [cluster $cluster_id; $($profile)?]
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
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum Command {
            $(
                /// ZCL command variant.
                $variant(std::boxed::Box<$command>),
            )+
        }

        impl Command {
            pub(crate) fn parse_zcl_frame<T>(
                header: $crate::Header,
                bytes: T,
            ) -> ::core::result::Result<Self, $crate::ParseFrameError>
            where
                T: ::core::iter::Iterator<Item = u8>,
            {
                match (header.command_id(), header.control().direction()) {
                    $(
                        (command_id, direction)
                            if command_id == <$command as $crate::Command>::ID
                                && <$command as $crate::Command>::PARSE_DIRECTION
                                    .accepts(direction) =>
                        {
                            <std::boxed::Box<$command> as le_stream::FromLeStream>::from_le_stream(
                                bytes,
                            )
                            .map(Self::$variant)
                            .ok_or($crate::ParseFrameError::InsufficientPayload)
                        }
                    )+
                    (command_id, _) => Err($crate::ParseFrameError::InvalidCommandId(command_id)),
                }
            }
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
    };
    (@cluster_impl [global]) => {};
    (@cluster_impl [cluster $cluster_id:expr; $($profile:expr)?]) => {
        impl zb_core::ClusterSpecific<zb_core::Cluster> for Command {
            const ID: zb_core::Cluster = $cluster_id;
        }

        impl zb_core::Profiled for Command {
            const PROFILE: zb_core::Profile =
                $crate::macros::zcl_cluster_profile!([$($profile)?]);
        }
    };
}

pub(crate) use zcl_command_enum;

macro_rules! zcl_attribute_newtype {
    (
        $(#[$attr:meta])*
        $vis:vis ranged struct $name:ident($inner:ty) = $min:literal..=$max:literal;
    ) => {
        $(#[$attr])*
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
            serde(transparent)
        )]
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            le_stream::ToLeStream,
        )]
        #[repr(transparent)]
        $vis struct $name($inner);

        impl $name {
            /// Minimum allowed value.
            pub const MIN: $inner = $min;

            /// Maximum allowed value.
            pub const MAX: $inner = $max;

            /// Try to create a new attribute value.
            #[must_use]
            pub fn try_new(value: $inner) -> Option<Self> {
                if (Self::MIN..=Self::MAX).contains(&value) {
                    Some(Self(value))
                } else {
                    None
                }
            }

            /// Return the inner value.
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl TryFrom<$inner> for $name {
            type Error = $inner;

            fn try_from(value: $inner) -> Result<Self, Self::Error> {
                Self::try_new(value).ok_or(value)
            }
        }

        impl le_stream::FromLeStream for $name {
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                <$inner as le_stream::FromLeStream>::from_le_stream(&mut bytes)
                    .and_then(|value| value.try_into().ok())
            }
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis bitflags $name:ident($inner:ty) => $variant:ident {
            $(
                $(#[$($flag_attr:tt)*])*
                const $flag:ident = $value:expr;
            )+
        }
    ) => {
        $(#[$attr])*
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
            serde(transparent)
        )]
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            le_stream::FromLeStream,
            le_stream::ToLeStream,
        )]
        #[repr(transparent)]
        $vis struct $name($inner);

        bitflags::bitflags! {
            impl $name: $inner {
                $(
                    $(#[$($flag_attr)*])*
                    const $flag = $value;
                )+
            }
        }

        $crate::macros::zcl_attribute_newtype! {
            @type_id_impl
            [$name]
            [$inner]
            [$variant]
        }

        impl From<$name> for zb_core::types::Type {
            fn from(value: $name) -> Self {
                Self::$variant(value.bits().into())
            }
        }

        impl TryFrom<zb_core::types::Type> for $name {
            type Error = zb_core::types::Type;

            fn try_from(value: zb_core::types::Type) -> Result<Self, Self::Error> {
                if let zb_core::types::Type::$variant(value) = value {
                    Ok(Self::from_bits_retain(value.into()))
                } else {
                    Err(value)
                }
            }
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident: Enum8 {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:expr,
            )+
        }
    ) => {
        $crate::macros::zcl_attribute_newtype! {
            @enum
            [$(#[$attr])*]
            [$vis]
            [$name]
            [Enum8]
            [
                $(
                    $(#[$variant_attr])*
                    $variant = $value,
                )+
            ]
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident: Map8 {
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:expr,
            )+
        }
    ) => {
        $crate::macros::zcl_attribute_newtype! {
            @enum
            [$(#[$attr])*]
            [$vis]
            [$name]
            [Map8]
            [
                $(
                    $(#[$variant_attr])*
                    $variant = $value,
                )+
            ]
        }
    };
    (
        @enum
        [$($attr:tt)*]
        [$vis:vis]
        [$name:ident]
        [$type_variant:ident]
        [
            $(
                $(#[$variant_attr:meta])*
                $variant:ident = $value:expr,
            )+
        ]
    ) => {
        $($attr)*
        #[allow(clippy::enum_variant_names)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive)]
        #[repr(u8)]
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant = $value,
            )+
        }

        impl From<$name> for u8 {
            fn from(value: $name) -> Self {
                value as Self
            }
        }

        impl From<$name> for zb_core::types::Uint8 {
            fn from(value: $name) -> Self {
                Self::new(value.into())
            }
        }

        $crate::macros::zcl_attribute_newtype! {
            @enum_from_type
            [$name]
            [$type_variant]
        }


        $crate::macros::zcl_attribute_newtype! {
            @type_id_impl
            [$name]
            [u8]
            [$type_variant]
        }

        impl TryFrom<u8> for $name {
            type Error = u8;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                num_traits::FromPrimitive::from_u8(value).ok_or(value)
            }
        }

        impl TryFrom<zb_core::types::Uint8> for $name {
            type Error = zb_core::types::Uint8;

            fn try_from(value: zb_core::types::Uint8) -> Result<Self, Self::Error> {
                Self::try_from(value.into_inner()).map_err(|_| value)
            }
        }

        $crate::macros::zcl_attribute_newtype! {
            @enum_try_from_type
            [$name]
            [$type_variant]
        }

        impl le_stream::FromLeStream for $name {
            fn from_le_stream<T>(mut bytes: T) -> Option<Self>
            where
                T: Iterator<Item = u8>,
            {
                u8::from_le_stream(&mut bytes).and_then(|value| Self::try_from(value).ok())
            }
        }

        impl le_stream::ToLeStream for $name {
            type Iter = <u8 as le_stream::ToLeStream>::Iter;

            fn to_le_stream(self) -> Self::Iter {
                u8::from(self).to_le_stream()
            }
        }
    };
    (@enum_from_type [$name:ident] [Enum8]) => {
        impl From<$name> for zb_core::types::Type {
            fn from(value: $name) -> Self {
                Self::Enum8(zb_core::types::Enum8::new(value.into()))
            }
        }
    };
    (@enum_from_type [$name:ident] [Map8]) => {
        impl From<$name> for zb_core::types::Type {
            fn from(value: $name) -> Self {
                Self::Map8(value.into())
            }
        }
    };
    (@enum_try_from_type [$name:ident] [Enum8]) => {
        impl TryFrom<zb_core::types::Type> for $name {
            type Error = zb_core::types::Type;

            fn try_from(value: zb_core::types::Type) -> Result<Self, Self::Error> {
                if let zb_core::types::Type::Enum8(value) = value {
                    Self::try_from(value.into_inner())
                        .map_err(|value| zb_core::types::Type::Enum8(value.into()))
                } else {
                    Err(value)
                }
            }
        }
    };
    (@enum_try_from_type [$name:ident] [Map8]) => {
        impl TryFrom<zb_core::types::Type> for $name {
            type Error = zb_core::types::Type;

            fn try_from(value: zb_core::types::Type) -> Result<Self, Self::Error> {
                if let zb_core::types::Type::Map8(raw) = value {
                    raw.try_into().map_err(|_| zb_core::types::Type::Map8(raw))
                } else {
                    Err(value)
                }
            }
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident($inner:ty) => $variant:ident;
    ) => {
        $(#[$attr])*
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize, serde::Deserialize),
            serde(transparent)
        )]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[repr(transparent)]
        $vis struct $name($inner);

        impl $name {
            /// Create a new attribute value.
            #[must_use]
            pub const fn new(value: $inner) -> Self {
                Self(value)
            }

            /// Return the inner value.
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }


        $crate::macros::zcl_attribute_newtype! {
            @type_id_impl
            [$name]
            [$inner]
            [$variant]
        }

        impl From<$name> for zb_core::types::Type {
            fn from(value: $name) -> Self {
                Self::$variant(value.0.into())
            }
        }

        impl TryFrom<zb_core::types::Type> for $name {
            type Error = zb_core::types::Type;

            fn try_from(value: zb_core::types::Type) -> Result<Self, Self::Error> {
                if let zb_core::types::Type::$variant(value) = value {
                    Ok(Self(value.into()))
                } else {
                    Err(value)
                }
            }
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [Enum8]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <zb_core::types::Enum8 as zb_core::TypeId>::ID;
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [Map8]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <u8 as zb_core::TypeId>::ID;
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [Map16]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <zb_core::types::Map16 as zb_core::TypeId>::ID;
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [Map32]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <zb_core::types::Map32 as zb_core::TypeId>::ID;
        }
    };
    (@type_id_impl [$name:ident] [$inner:ty] [$variant:ident]) => {
        impl zb_core::TypeId for $name {
            const ID: u8 = <$inner as zb_core::TypeId>::ID;
        }
    };
}

#[allow(unused_imports)]
pub(crate) use zcl_attribute_newtype;

/// Define access-specific ZCL attribute enums from one attribute table.
///
/// The macro generates fixed enum names in the invocation module:
/// `Id` for readable attribute IDs, plus `Readable`, `Writable`,
/// `Reportable`, and `Scene` for access-specific attribute values. `SendReport`
/// associates reportable attributes with their ZCL wire types. The
/// cluster ID is required and is used to implement `Cluster` for the generated
/// enums. The global readable attributes `ClusterRevision` and
/// `AttributeReportingStatus` are always included.
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
        $(profile: $profile:expr;)?
        manufacturer_code: $manufacturer_code:expr;
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ident $(<$ty_argument:tt>)? {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            [cluster $cluster_id; $($profile)?]
            [$manufacturer_code]
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty $(<$ty_argument>)? {
                    $($access)*
                }
            ),*
        }
    };
    (
        cluster: $cluster_id:expr;
        $(profile: $profile:expr;)?
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ident $(<$ty_argument:tt>)? {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            [cluster $cluster_id; $($profile)?]
            []
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty $(<$ty_argument>)? {
                    $($access)*
                }
            ),*
        }
    };
    (
        { $cluster_id:expr };
        $(profile: $profile:expr;)?
        manufacturer_code: $manufacturer_code:expr;
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ident $(<$ty_argument:tt>)? {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            [cluster $cluster_id; $($profile)?]
            [$manufacturer_code]
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty $(<$ty_argument>)? {
                    $($access)*
                }
            ),*
        }
    };
    (
        { $cluster_id:expr };
        $(profile: $profile:expr;)?
        $(
            $(#[$variant_attr:meta])*
            $variant:ident = $id:tt: $ty:ident $(<$ty_argument:tt>)? {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define
            [cluster $cluster_id; $($profile)?]
            []
            $(
                $(#[$variant_attr])*
                $variant = $id: $ty $(<$ty_argument>)? {
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
            $variant:ident = $id:tt: $ty:ident $(<$ty_argument:tt>)? {
                $($access:tt)*
            }
        ),* $(,)?
    ) => {
        $crate::macros::zcl_attributes! {
            @define_readable
            $cluster
            [$($manufacturer_code)?]
            [
                /// The revision of the cluster specification that the cluster instance supports.
                ClusterRevision = 0xfffd,
                /// The reporting status of the cluster instance.
                AttributeReportingStatus = 0xfffe,
            ]
            [
                /// The revision of the cluster specification that the cluster instance supports.
                ClusterRevision(zb_core::types::Uint16) = 0xfffd,
                /// The reporting status of the cluster instance.
                AttributeReportingStatus(zb_core::types::Uint8) = 0xfffe,
            ]
            [
                Id::ClusterRevision => 0xfffd,
                Id::AttributeReportingStatus => 0xfffe,
            ]
            [
                0xfffd => Ok(Id::ClusterRevision),
                0xfffe => Ok(Id::AttributeReportingStatus),
            ]
            [
                (Id::ClusterRevision, typ) => <zb_core::types::Uint16 as TryFrom<zb_core::types::Type>>::try_from(typ).map(Readable::ClusterRevision).map_err(Into::into),
                (Id::AttributeReportingStatus, typ) => <zb_core::types::Uint8 as TryFrom<zb_core::types::Type>>::try_from(typ).map(Readable::AttributeReportingStatus).map_err(Into::into),
            ]
            [
                Readable::ClusterRevision(value) => value.into(),
                Readable::AttributeReportingStatus(value) => value.into(),
            ]
            [$([$(#[$variant_attr])*] [$variant] [$id] [$ty $(<$ty_argument>)?] [$($access)*];)*]
        }

        $crate::macros::zcl_attributes! {
            @define_writable
            $cluster
            [$($manufacturer_code)?]
            [] [] []
            [$([$(#[$variant_attr])*] [$variant] [$id] [$ty $(<$ty_argument>)?] [$($access)*];)*]
        }

        $crate::macros::zcl_attributes! {
            @define_reportable
            $cluster
            [$($manufacturer_code)?]
            [] []
            [$([$(#[$variant_attr])*] [$variant] [$id] [$ty $(<$ty_argument>)?] [$($access)*];)*]
        }

        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Scene]
            ["Attributes that can be stored in scenes."]
            [S]
            []
            [$([$(#[$variant_attr])*] [$variant] [$id] [$ty $(<$ty_argument>)?] [$($access)*];)*]
        }

        const _: () = {
            const fn assert_type_id<T>()
            where
                T: zb_core::TypeId,
            {
            }

            $(let _ = assert_type_id::<$ty $(<$ty_argument>)?>;)*
        };
    };
    (@manufacturer_code []) => {};
    (@manufacturer_code [$manufacturer_code:expr]) => {
        const MANUFACTURER_CODE: Option<u16> = Some($manufacturer_code);
    };
    (@cluster_impl [cluster $cluster_id:expr; $($profile:expr)?] [$($manufacturer_code:expr)?] $ty:ident) => {
        impl zb_core::ClusterSpecific<zb_core::Cluster> for $ty {
            const ID: zb_core::Cluster = $cluster_id;
        }

        impl zb_core::Profiled for $ty {
            const PROFILE: zb_core::Profile =
                $crate::macros::zcl_cluster_profile!([$($profile)?]);
        }
    };
    (@readable_attribute_impl [$($manufacturer_code:expr)?]) => {
        impl $crate::Readable for Id {
            $crate::macros::zcl_attributes! {
                @manufacturer_code [$($manufacturer_code)?]
            }

            type Attribute = Readable;
        }
    };
    (
        @writable_attribute_impl
        [$($manufacturer_code:expr)?]
        []
    ) => {};
    (
        @writable_attribute_impl
        [$($manufacturer_code:expr)?]
        [$($id_arms:tt)+]
    ) => {
        impl $crate::Writable for Writable {
            $crate::macros::zcl_attributes! {
                @manufacturer_code [$($manufacturer_code)?]
            }

            fn id(&self) -> u16 {
                match self {
                    $($id_arms)+
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
            [$($manufacturer_code)?]
            Id
        }

        $crate::macros::zcl_attributes! {
            @cluster_impl
            $cluster
            [$($manufacturer_code)?]
            Readable
        }

        $crate::macros::zcl_attributes! {
            @readable_attribute_impl
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

        impl TryFrom<(Id, zb_core::types::Type)> for Readable {
            type Error = $crate::InvalidType<Id>;

            fn try_from(
                (id, typ): (Id, zb_core::types::Type),
            ) -> Result<Self, Self::Error> {
                match (id, typ) {
                    $($try_from_readable_arms)*
                }
                .map_err(|typ| $crate::InvalidType::new(id, typ))
            }
        }

        impl From<Readable> for zb_core::types::Type {
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
            [$($try_from_readable_arms)* (Id::$variant, typ) => <$ty as TryFrom<zb_core::types::Type>>::try_from(typ).map(Readable::$variant).map_err(Into::into),]
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
            [$($manufacturer_code)?]
            Writable
        }

        $crate::macros::zcl_attributes! {
            @writable_attribute_impl
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
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, W $(, $($access_tail:tt)*)?]; $($rest:tt)*]
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
        @define_writable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($writable_variants:tt)*]
        [$($id_arms:tt)*]
        [$($record_arms:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R $(, $($access_tail:tt)*)?]; $($rest:tt)*]
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
        [R, W $(, $($tail:tt)*)?]
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
        @define_reportable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($variants:tt)*]
        []
        []
    ) => {
        $crate::macros::zcl_attributes! {
            @emit_value_enum
            [Reportable]
            ["Attributes that can be reported."]
            [$($variants)*]
        }

        $crate::macros::zcl_attributes! {
            @emit_send_report_enum
            [$($manufacturer_code)?]
            [$($variants)*]
        }

        $crate::macros::zcl_attributes! {
            @cluster_impl
            $cluster
            [$($manufacturer_code)?]
            Reportable
        }

        $crate::macros::zcl_attributes! {
            @cluster_impl
            $cluster
            [$($manufacturer_code)?]
            SendReport
        }

        impl TryFrom<(u16, zb_core::types::Type)> for Reportable {
            type Error = $crate::ParseAttributeError<u16>;

            fn try_from(
                (id, _typ): (u16, zb_core::types::Type),
            ) -> Result<Self, Self::Error> {
                Err($crate::ParseAttributeError::InvalidId(id))
            }
        }
    };
    (
        @define_reportable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($variants:tt)*]
        [$([$try_variant:ident] [$try_id:tt] [$try_ty:ty];)+]
        []
    ) => {
        $crate::macros::zcl_attributes! {
            @emit_value_enum
            [Reportable]
            ["Attributes that can be reported."]
            [$($variants)*]
        }

        $crate::macros::zcl_attributes! {
            @emit_send_report_enum
            [$($manufacturer_code)?]
            [$($variants)*]
        }

        $crate::macros::zcl_attributes! {
            @cluster_impl
            $cluster
            [$($manufacturer_code)?]
            Reportable
        }

        $crate::macros::zcl_attributes! {
            @cluster_impl
            $cluster
            [$($manufacturer_code)?]
            SendReport
        }

        impl TryFrom<(u16, zb_core::types::Type)> for Reportable {
            type Error = $crate::ParseAttributeError<u16>;

            fn try_from(
                (id, typ): (u16, zb_core::types::Type),
            ) -> Result<Self, Self::Error> {
                match id {
                    $(
                        $try_id => <$try_ty as TryFrom<zb_core::types::Type>>::try_from(typ)
                            .map(Reportable::$try_variant)
                            .map_err(|typ| $crate::InvalidType::new(id, typ).into()),
                    )+
                    other => Err($crate::ParseAttributeError::InvalidId(other)),
                }
            }
        }
    };
    (
        @define_reportable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($variants:tt)*]
        [$($try_from_arms:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ident] [R, P $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_reportable
            $cluster
            [$($manufacturer_code)?]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($try_from_arms)* [$variant] [$id] [$ty];]
            [$($rest)*]
        }
    };
    (
        @define_reportable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($variants:tt)*]
        [$($try_from_arms:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ident] [R, W, P $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_reportable
            $cluster
            [$($manufacturer_code)?]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($try_from_arms)* [$variant] [$id] [$ty];]
            [$($rest)*]
        }
    };
    (
        @define_reportable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($variants:tt)*]
        [$($try_from_arms:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ident] [W, P $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_reportable
            $cluster
            [$($manufacturer_code)?]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($try_from_arms)* [$variant] [$id] [$ty];]
            [$($rest)*]
        }
    };
    (
        @define_reportable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($variants:tt)*]
        [$($try_from_arms:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ident] [P $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_reportable
            $cluster
            [$($manufacturer_code)?]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($try_from_arms)* [$variant] [$id] [$ty];]
            [$($rest)*]
        }
    };
    (
        @define_reportable
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$($variants:tt)*]
        [$($try_from_arms:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [$($access:tt)*]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_reportable
            $cluster
            [$($manufacturer_code)?]
            [$($variants)*]
            [$($try_from_arms)*]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$enum:ident]
        [$doc:literal]
        [$access:tt]
        [$($variants:tt)*]
        []
    ) => {
        $crate::macros::zcl_attributes! { @emit_value_enum [$enum] [$doc] [$($variants)*] }

        $crate::macros::zcl_attributes! {
            @cluster_impl
            $cluster
            [$($manufacturer_code)?]
            $enum
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Reportable]
        [$doc:literal]
        [P]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, P $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Reportable]
            [$doc]
            [P]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Reportable]
        [$doc:literal]
        [P]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, W, P $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Reportable]
            [$doc]
            [P]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Scene]
        [$doc:literal]
        [S]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, S $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Scene]
            [$doc]
            [S]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Scene]
        [$doc:literal]
        [S]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, P, S $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Scene]
            [$doc]
            [S]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Scene]
        [$doc:literal]
        [S]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, W, S $(, $($access_tail:tt)*)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Scene]
            [$doc]
            [S]
            [$($variants)* $($variant_attr)* $variant($ty) = $id,]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Reportable]
        [$doc:literal]
        [P]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Reportable]
            [$doc]
            [P]
            [$($variants)*]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Reportable]
        [$doc:literal]
        [P]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, W $(, S)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Reportable]
            [$doc]
            [P]
            [$($variants)*]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Scene]
        [$doc:literal]
        [S]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Scene]
            [$doc]
            [S]
            [$($variants)*]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [Scene]
        [$doc:literal]
        [S]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, W $(, P)?]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @define_data_enum
            $cluster
            [$($manufacturer_code)?]
            [Scene]
            [$doc]
            [S]
            [$($variants)*]
            [$($rest)*]
        }
    };
    (
        @define_data_enum
        $cluster:tt
        [$($manufacturer_code:expr)?]
        [$enum:ident]
        [$doc:literal]
        [$access:tt]
        [$($variants:tt)*]
        [[$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [$($flags:tt)*]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @data_access
            $cluster
            [$($manufacturer_code)?]
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
    (@data_access $cluster:tt [$($manufacturer_code:expr)?] [$enum:ident] [$doc:literal] [$access:tt] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] []) => {
        $crate::macros::zcl_attributes! { @define_data_enum $cluster [$($manufacturer_code)?] [$enum] [$doc] [$access] [$($variants)*] [$($rest)*] }
    };
    (@data_access $cluster:tt [$($manufacturer_code:expr)?] [Reportable] [$doc:literal] [P] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, P $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @define_data_enum $cluster [$($manufacturer_code)?] [Reportable] [$doc] [P] [$($variants)* $($variant_attr)* $variant($ty) = $id,] [$($rest)*] }
    };
    (@data_access $cluster:tt [$($manufacturer_code:expr)?] [Scene] [$doc:literal] [S] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, S $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @define_data_enum $cluster [$($manufacturer_code)?] [Scene] [$doc] [S] [$($variants)* $($variant_attr)* $variant($ty) = $id,] [$($rest)*] }
    };
    (@data_access $cluster:tt [$($manufacturer_code:expr)?] [Scene] [$doc:literal] [S] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [R, P, S $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @define_data_enum $cluster [$($manufacturer_code)?] [Scene] [$doc] [S] [$($variants)* $($variant_attr)* $variant($ty) = $id,] [$($rest)*] }
    };
    (@data_access $cluster:tt [$($manufacturer_code:expr)?] [Reportable] [$doc:literal] [P] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [P $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @define_data_enum $cluster [$($manufacturer_code)?] [Reportable] [$doc] [P] [$($variants)* $($variant_attr)* $variant($ty) = $id,] [$($rest)*] }
    };
    (@data_access $cluster:tt [$($manufacturer_code:expr)?] [Scene] [$doc:literal] [S] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [S $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @define_data_enum $cluster [$($manufacturer_code)?] [Scene] [$doc] [S] [$($variants)* $($variant_attr)* $variant($ty) = $id,] [$($rest)*] }
    };
    (@data_access $cluster:tt [$($manufacturer_code:expr)?] [$enum:ident] [$doc:literal] [$access:tt] [$($variants:tt)*] [$($rest:tt)*] [$($variant_attr:tt)*] [$variant:ident] [$id:tt] [$ty:ty] [$ignored:tt $(, $($tail:tt)*)?]) => {
        $crate::macros::zcl_attributes! { @data_access $cluster [$($manufacturer_code)?] [$enum] [$doc] [$access] [$($variants)*] [$($rest)*] [$($variant_attr)*] [$variant] [$id] [$ty] [$($($tail)*)?] }
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
    (@emit_send_report_enum [$($manufacturer_code:expr)?] []) => {
        /// ZCL wire types associated with reportable attributes.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum SendReport {}

        impl $crate::Reportable for SendReport {
            $crate::macros::zcl_attributes! {
                @manufacturer_code [$($manufacturer_code)?]
            }

            fn attribute_id(&self) -> u16 {
                unreachable!("an empty SendReport enum cannot be instantiated")
            }

            fn type_id(&self) -> u8 {
                unreachable!("an empty SendReport enum cannot be instantiated")
            }
        }

        impl From<SendReport>
            for $crate::global::configure_reporting::send::AttributeReportingConfiguration
        {
            fn from(value: SendReport) -> Self {
                match value {}
            }
        }
    };
    (
        @emit_send_report_enum
        [$($manufacturer_code:expr)?]
        [
            $(
                $(#[$variant_attr:meta])*
                $variant:ident($ty:ident) = $id:tt,
            )+
        ]
    ) => {
        $crate::macros::zcl_attributes! {
            @classify_send_report_variants
            [$($manufacturer_code)?]
            []
            []
            []
            [
                $(
                    [$(#[$variant_attr])*] [$variant] [$ty] [$id];
                )+
            ]
        }
    };
    (
        @classify_send_report_variants
        $manufacturer_code:tt
        [$($variants:tt)*]
        [$($type_id_arms:tt)*]
        [$($conversion_arms:tt)*]
        []
    ) => {
        /// ZCL wire types associated with reportable attributes.
        #[derive(
            Clone,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            repr_discriminant::ReprDiscriminant,
        )]
        #[repr(u16)]
        pub enum SendReport {
            $($variants)*
        }

        impl $crate::Reportable for SendReport {
            $crate::macros::zcl_attributes! {
                @manufacturer_code $manufacturer_code
            }

            fn attribute_id(&self) -> u16 {
                repr_discriminant::ReprDiscriminant::repr_discriminant(self)
            }

            fn type_id(&self) -> u8 {
                match self {
                    $($type_id_arms)*
                }
            }
        }

        impl From<SendReport>
            for $crate::global::configure_reporting::send::AttributeReportingConfiguration
        {
            fn from(value: SendReport) -> Self {
                match value {
                    $($conversion_arms)*
                }
            }
        }
    };
    (
        @classify_send_report_variants
        $manufacturer_code:tt
        $variants:tt
        $type_id_arms:tt
        $conversion_arms:tt
        [[$($variant_attr:tt)*] [$variant:ident] [$ty:ident] [$id:tt]; $($rest:tt)*]
    ) => {
        $crate::macros::zcl_attributes! {
            @classify_send_report_variant
            $manufacturer_code
            $variants
            $type_id_arms
            $conversion_arms
            [$($rest)*]
            [$($variant_attr)*]
            [$variant]
            [$ty]
            [$id]
        }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Uint8] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Uint8] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Uint16] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Uint16] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Uint24] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Uint24] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Uint32] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Uint32] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Uint40] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Uint40] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Uint48] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Uint48] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Uint56] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Uint56] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Uint64] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Uint64] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Int8] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Int8] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Int16] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Int16] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Int24] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Int24] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Int32] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Int32] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Int40] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Int40] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Int48] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Int48] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Int56] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Int56] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Int64] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Int64] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [TimeOfDay] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [TimeOfDay] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Date] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Date] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [UtcTime] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [UtcTime] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [Mireds] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [Mireds] $id }
    };
    (@classify_send_report_variant $manufacturer_code:tt $variants:tt $type_id_arms:tt $conversion_arms:tt $rest:tt $attrs:tt $variant:tt [MeasuredValue] $id:tt) => {
        $crate::macros::zcl_attributes! { @send_report_analog $manufacturer_code $variants $type_id_arms $conversion_arms $rest $attrs $variant [MeasuredValue] $id }
    };
    (
        @classify_send_report_variant
        $manufacturer_code:tt
        $variants:tt
        $type_id_arms:tt
        $conversion_arms:tt
        $rest:tt
        $attrs:tt
        $variant:tt
        [$ty:ty]
        $id:tt
    ) => {
        $crate::macros::zcl_attributes! {
            @send_report_discrete
            $manufacturer_code
            $variants
            $type_id_arms
            $conversion_arms
            $rest
            $attrs
            $variant
            [$ty]
            $id
        }
    };
    (
        @send_report_analog
        $manufacturer_code:tt
        [$($variants:tt)*]
        [$($type_id_arms:tt)*]
        [$($conversion_arms:tt)*]
        [$($rest:tt)*]
        [$($variant_attr:tt)*]
        [$variant:ident]
        [$ty:ty]
        [$id:tt]
    ) => {
        $crate::macros::zcl_attributes! {
            @classify_send_report_variants
            $manufacturer_code
            [
                $($variants)*
                $($variant_attr)*
                $variant($crate::Analog<$ty>) = $id,
            ]
            [
                $($type_id_arms)*
                SendReport::$variant(_) => <$ty as zb_core::TypeId>::ID,
            ]
            [
                $($conversion_arms)*
                SendReport::$variant(value) => Self::analog($id, value),
            ]
            [$($rest)*]
        }
    };
    (
        @send_report_discrete
        $manufacturer_code:tt
        [$($variants:tt)*]
        [$($type_id_arms:tt)*]
        [$($conversion_arms:tt)*]
        [$($rest:tt)*]
        [$($variant_attr:tt)*]
        [$variant:ident]
        [$ty:ty]
        [$id:tt]
    ) => {
        $crate::macros::zcl_attributes! {
            @classify_send_report_variants
            $manufacturer_code
            [
                $($variants)*
                $($variant_attr)*
                $variant($crate::Discrete<$ty>) = $id,
            ]
            [
                $($type_id_arms)*
                SendReport::$variant(_) => <$ty as zb_core::TypeId>::ID,
            ]
            [
                $($conversion_arms)*
                SendReport::$variant(value) => Self::discrete($id, &value),
            ]
            [$($rest)*]
        }
    };
    (@emit_value_enum [$enum:ident] [$doc:literal] []) => {
        #[doc = $doc]
        #[allow(dead_code)]
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub enum $enum {}
    };
    (@emit_value_enum [$enum:ident] [$doc:literal] [$($variants:tt)+]) => {
        #[doc = $doc]
        #[allow(dead_code)]
        #[allow(clippy::large_enum_variant, variant_size_differences)]
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
    use zb_core::types::{Type, Uint8};
    use zb_core::{Cluster, Profile, Profiled};

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Custom(Uint8);

    impl zb_core::TypeId for Custom {
        const ID: u8 = <Uint8 as zb_core::TypeId>::ID;
    }

    impl From<Custom> for Type {
        fn from(value: Custom) -> Self {
            value.0.into()
        }
    }

    zcl_attributes! {
        cluster: Cluster::OnOff;
        profile: Profile::TouchLink;
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
        assert_eq!(<Id as Profiled>::PROFILE, Profile::TouchLink);
        assert_eq!(<Readable as Profiled>::PROFILE, Profile::TouchLink);
        assert_eq!(<Id as crate::Readable>::MANUFACTURER_CODE, Some(0x1234));
        assert_eq!(<Writable as Profiled>::PROFILE, Profile::TouchLink);
        assert_eq!(
            <Writable as crate::Writable>::MANUFACTURER_CODE,
            Some(0x1234)
        );
        assert_eq!(<Reportable as Profiled>::PROFILE, Profile::TouchLink);
        assert_eq!(<SendReport as Profiled>::PROFILE, Profile::TouchLink);
        assert_eq!(
            <SendReport as crate::Reportable>::MANUFACTURER_CODE,
            Some(0x1234)
        );
        assert_eq!(<Scene as Profiled>::PROFILE, Profile::TouchLink);
        assert_eq!(
            <Custom as zb_core::TypeId>::ID,
            <Uint8 as zb_core::TypeId>::ID
        );

        let _ = Id::ReadOnly;
        let _ = Id::ClusterRevision;
        let _ = Id::AttributeReportingStatus;
        let _ = Readable::ReadOnly(Uint8::new(1));
        let _ = Readable::Writable(Uint8::new(2));
        let _ = Readable::ClusterRevision(zb_core::types::Uint16::new(1));
        let _ = Readable::AttributeReportingStatus(Uint8::new(0));
        let _ = Writable::Writable(Uint8::new(3));
        let _ = Writable::WriteOnly(Custom(Uint8::new(4)));
        let _ = Reportable::Writable(Uint8::new(5));
        assert_eq!(
            crate::Reportable::attribute_id(&SendReport::Writable(crate::Analog::new(
                1,
                2,
                Uint8::new(5),
            ))),
            0x0001
        );
        assert_eq!(
            crate::Reportable::type_id(&SendReport::Writable(crate::Analog::new(
                1,
                2,
                Uint8::new(5),
            ))),
            0x20
        );
        let _ = Scene::Writable(Uint8::new(6));
    }

    mod required_cluster {
        use super::{Cluster, Profile, Profiled, Uint8};

        zcl_attributes! {
            cluster: Cluster::Basic;

            /// Read-only test attribute.
            ReadOnly = 0x0000: Uint8 { R },
            /// Writable test attribute.
            Writable = 0x0001: Uint8 { W, P, S },
        }

        #[test]
        fn generates_cluster_bound_impls() {
            fn assert_readable<T>()
            where
                T: zb_core::ClusterSpecific<Cluster> + crate::Readable,
            {
            }

            fn assert_writable<T>()
            where
                T: zb_core::ClusterSpecific<Cluster> + crate::Writable,
            {
            }

            fn assert_cluster<T>()
            where
                T: zb_core::ClusterSpecific<Cluster>,
            {
            }

            fn assert_reportable<T>()
            where
                T: crate::Reportable,
            {
            }

            assert_readable::<Id>();
            assert_writable::<Writable>();
            assert_cluster::<Readable>();
            assert_cluster::<Reportable>();
            assert_cluster::<SendReport>();
            assert_reportable::<SendReport>();
            assert_cluster::<Scene>();

            assert_eq!(<Id as Profiled>::PROFILE, Profile::ZigbeeHomeAutomation);
            assert_eq!(
                <Readable as Profiled>::PROFILE,
                Profile::ZigbeeHomeAutomation
            );
            assert_eq!(
                <Writable as Profiled>::PROFILE,
                Profile::ZigbeeHomeAutomation
            );
            assert_eq!(
                <Reportable as Profiled>::PROFILE,
                Profile::ZigbeeHomeAutomation
            );
            assert_eq!(
                <SendReport as Profiled>::PROFILE,
                Profile::ZigbeeHomeAutomation
            );
            assert_eq!(<SendReport as crate::Reportable>::MANUFACTURER_CODE, None);
            assert_eq!(<Scene as Profiled>::PROFILE, Profile::ZigbeeHomeAutomation);

            let _ = Id::ReadOnly;
            let _ = Id::ClusterRevision;
            let _ = Id::AttributeReportingStatus;
            let _ = Readable::ReadOnly(Uint8::new(1));
            let _ = Readable::ClusterRevision(zb_core::types::Uint16::new(1));
            let _ = Readable::AttributeReportingStatus(Uint8::new(0));
            let _ = Writable::Writable(Uint8::new(2));
            let _ = Reportable::Writable(Uint8::new(3));
            assert_eq!(
                crate::Reportable::type_id(&SendReport::Writable(crate::Analog::new(
                    1,
                    2,
                    Uint8::new(3),
                ))),
                0x20
            );
            let _ = Scene::Writable(Uint8::new(4));
        }
    }
}
