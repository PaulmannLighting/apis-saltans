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
