//! ZDP services.

use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};

pub use self::bind_management::{
    BindManagement, BindReq, BindRsp, ClearAllBindingsReq, Destination, UnbindReq,
};
pub use self::device_and_service_discovery::{
    ActiveEpReq, ActiveEpRsp, DeviceAndServiceDiscovery, DeviceAnnce, IeeeAddrReq, MatchDescReq,
    MatchDescRsp, NodeDescReq, NodeDescRsp, NwkAddrReq, ParentAnnce, PowerDescReq, RequestType,
    SimpleDescReq, SimpleDescRsp, SystemServerDiscoveryReq,
};
pub use self::network_management::{
    EnhancedNwkUpdateParameters, LeaveReqFlags, MgmtBindReq, MgmtLeaveReq, MgmtLqiReq,
    MgmtNwkBeaconSurveyReq, MgmtNwkEnhancedUpdateReq, MgmtNwkIeeeJoiningListReq, MgmtNwkUpdateReq,
    MgmtPermitJoiningReq, MgmtPermitJoiningRsp, MgmtRtgReq, NetworkManagement, ScanDuration,
};

mod bind_management;
mod device_and_service_discovery;
mod network_management;

/// A ZDP client service.
pub trait Service {
    /// The name of the service.
    const NAME: &'static str;
}

macro_rules! zdp_command {
    (
        $(#[$attribute:meta])*
        derive { $($extra_derive:path),* $(,)? }
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @stream
            [$($attribute),*]
            [$($extra_derive),*]
            $command => $name;
            cluster_id: $cluster_id;
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
        }
    };
    (
        $(#[$attribute:meta])*
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @stream
            [$($attribute),*]
            []
            $command => $name;
            cluster_id: $cluster_id;
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
        }
    };
    (
        $(#[$attribute:meta])*
        derive { $($extra_derive:path),* $(,)? }
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @stream
            [$($attribute),*]
            [$($extra_derive),*]
            $command => $name;
            cluster_id: $cluster_id;
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
        }
    };
    (
        $(#[$attribute:meta])*
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @stream
            [$($attribute),*]
            []
            $command => $name;
            cluster_id: $cluster_id;
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
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            [le_stream::FromLeStream, le_stream::ToLeStream]
            $command => $name;
            cluster_id: $cluster_id;
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
            le_stream {
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            [le_stream::ToLeStream]
            $command => $name;
            cluster_id: $cluster_id;
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
            le_stream {
                $($from_le_stream)*
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            [le_stream::FromLeStream]
            $command => $name;
            cluster_id: $cluster_id;
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
            le_stream {
                $($to_le_stream)*
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            []
            $command => $name;
            cluster_id: $cluster_id;
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
            le_stream {
                $($from_le_stream)*
                $($to_le_stream)*
            }
        }
    };
    (
        @stream
        [$($attribute:meta),*]
        [$($extra_derive:path),*]
        $command:ident => $name:ident;
        cluster_id: $cluster_id:expr;
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
    ) => {
        $crate::services::zdp_command! {
            @emit
            [$($attribute),*]
            [$($extra_derive),*]
            []
            $command => $name;
            cluster_id: $cluster_id;
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
            le_stream {
                $($to_le_stream)*
                $($from_le_stream)*
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
            $($le_stream:tt)*
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

        impl apis_saltans_core::Cluster for $command {
            const ID: u16 = Self::ID;
        }

        impl $crate::services::Service for $command {
            const NAME: &'static str = Self::NAME;
        }

        $crate::services::zdp_command! {
            @response
            $command
            $($response)?
        }

        impl std::fmt::Display for $command {
            $crate::services::zdp_command! {
                @display
                self,
                f,
                [$($field),*],
                {
                    $($display)*
                }
            }
        }

        $($le_stream)*
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

/// Available ZDP commands.
// TODO: Implement all commands.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Command {
    /// Device and Service Discovery Commands
    DeviceAndServiceDiscovery(DeviceAndServiceDiscovery),

    /// Bind Management Commands
    BindManagement(BindManagement),

    /// Network Management Commands
    NetworkManagement(NetworkManagement),
}

impl Command {
    /// Parses a ZDP command from the given cluster ID and byte stream.
    pub(crate) fn parse_with_cluster_id<T>(cluster_id: u16, bytes: T) -> Result<Option<Self>, u16>
    where
        T: Iterator<Item = u8>,
    {
        match cluster_id {
            // Device and Service Discovery Commands
            NwkAddrReq::ID => Ok(NwkAddrReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::NwkAddrReq)
                .map(Self::DeviceAndServiceDiscovery)),
            IeeeAddrReq::ID => Ok(IeeeAddrReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::IeeeAddrReq)
                .map(Self::DeviceAndServiceDiscovery)),
            NodeDescReq::ID => Ok(NodeDescReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::NodeDescReq)
                .map(Self::DeviceAndServiceDiscovery)),
            NodeDescRsp::ID => Ok(NodeDescRsp::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::NodeDescRsp)
                .map(Self::DeviceAndServiceDiscovery)),
            PowerDescReq::ID => Ok(PowerDescReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::PowerDescReq)
                .map(Self::DeviceAndServiceDiscovery)),
            SimpleDescReq::ID => Ok(SimpleDescReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::SimpleDescReq)
                .map(Self::DeviceAndServiceDiscovery)),
            SimpleDescRsp::ID => Ok(SimpleDescRsp::from_le_stream(bytes)
                .map(Box::new)
                .map(DeviceAndServiceDiscovery::SimpleDescRsp)
                .map(Self::DeviceAndServiceDiscovery)),
            ActiveEpReq::ID => Ok(ActiveEpReq::from_le_stream(bytes)
                .map(Box::new)
                .map(DeviceAndServiceDiscovery::ActiveEpReq)
                .map(Self::DeviceAndServiceDiscovery)),
            ActiveEpRsp::ID => Ok(ActiveEpRsp::from_le_stream(bytes)
                .map(Box::new)
                .map(DeviceAndServiceDiscovery::ActiveEpRsp)
                .map(Self::DeviceAndServiceDiscovery)),
            MatchDescReq::ID => Ok(MatchDescReq::from_le_stream(bytes)
                .map(Box::new)
                .map(DeviceAndServiceDiscovery::MatchDescReq)
                .map(Self::DeviceAndServiceDiscovery)),
            MatchDescRsp::ID => Ok(MatchDescRsp::from_le_stream(bytes)
                .map(Box::new)
                .map(DeviceAndServiceDiscovery::MatchDescRsp)
                .map(Self::DeviceAndServiceDiscovery)),
            DeviceAnnce::ID => Ok(DeviceAnnce::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::DeviceAnnce)
                .map(Self::DeviceAndServiceDiscovery)),
            ParentAnnce::ID => Ok(ParentAnnce::from_le_stream(bytes)
                .map(Box::new)
                .map(DeviceAndServiceDiscovery::ParentAnnce)
                .map(Self::DeviceAndServiceDiscovery)),
            SystemServerDiscoveryReq::ID => Ok(SystemServerDiscoveryReq::from_le_stream(bytes)
                .map(DeviceAndServiceDiscovery::SystemServerDiscoveryReq)
                .map(Self::DeviceAndServiceDiscovery)),

            // Bind Management Commands
            BindReq::ID => Ok(BindReq::from_le_stream(bytes)
                .map(BindManagement::BindReq)
                .map(Self::BindManagement)),
            BindRsp::ID => Ok(BindRsp::from_le_stream(bytes)
                .map(BindManagement::BindRsp)
                .map(Self::BindManagement)),
            UnbindReq::ID => Ok(UnbindReq::from_le_stream(bytes)
                .map(BindManagement::UnbindReq)
                .map(Self::BindManagement)),
            ClearAllBindingsReq::ID => Ok(ClearAllBindingsReq::from_le_stream(bytes)
                .map(BindManagement::ClearAllBindingsReq)
                .map(Self::BindManagement)),

            // Network Management Commands
            MgmtLqiReq::ID => Ok(MgmtLqiReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtLqiReq)
                .map(Self::NetworkManagement)),
            MgmtRtgReq::ID => Ok(MgmtRtgReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtRtgReq)
                .map(Self::NetworkManagement)),
            MgmtBindReq::ID => Ok(MgmtBindReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtBindReq)
                .map(Self::NetworkManagement)),
            MgmtLeaveReq::ID => Ok(MgmtLeaveReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtLeaveReq)
                .map(Self::NetworkManagement)),
            MgmtPermitJoiningReq::ID => Ok(MgmtPermitJoiningReq::from_le_stream(bytes)
                .map(Box::new)
                .map(NetworkManagement::MgmtPermitJoiningReq)
                .map(Self::NetworkManagement)),
            MgmtNwkUpdateReq::ID => Ok(MgmtNwkUpdateReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtNwkUpdateReq)
                .map(Self::NetworkManagement)),
            MgmtNwkEnhancedUpdateReq::ID => Ok(MgmtNwkEnhancedUpdateReq::from_le_stream(bytes)
                .map(Box::new)
                .map(NetworkManagement::MgmtNwkEnhancedUpdateReq)
                .map(Self::NetworkManagement)),
            MgmtNwkIeeeJoiningListReq::ID => Ok(MgmtNwkIeeeJoiningListReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtNwkIeeeJoiningListReq)
                .map(Self::NetworkManagement)),
            MgmtNwkBeaconSurveyReq::ID => Ok(MgmtNwkBeaconSurveyReq::from_le_stream(bytes)
                .map(NetworkManagement::MgmtNwkBeaconSurveyReq)
                .map(Self::NetworkManagement)),
            MgmtPermitJoiningRsp::ID => Ok(MgmtPermitJoiningRsp::from_le_stream(bytes)
                .map(NetworkManagement::MgmtPermitJoiningRsp)
                .map(Self::NetworkManagement)),
            other => Err(other),
        }
    }

    /// Return the cluster ID of the command.
    #[must_use]
    pub const fn cluster_id(&self) -> u16 {
        match self {
            Self::DeviceAndServiceDiscovery(cmd) => cmd.cluster_id(),
            Self::BindManagement(cmd) => cmd.cluster_id(),
            Self::NetworkManagement(cmd) => cmd.cluster_id(),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceAndServiceDiscovery(cmd) => cmd.fmt(f),
            Self::BindManagement(cmd) => cmd.fmt(f),
            Self::NetworkManagement(cmd) => cmd.fmt(f),
        }
    }
}

impl From<DeviceAndServiceDiscovery> for Command {
    fn from(cmd: DeviceAndServiceDiscovery) -> Self {
        Self::DeviceAndServiceDiscovery(cmd)
    }
}

impl From<BindManagement> for Command {
    fn from(cmd: BindManagement) -> Self {
        Self::BindManagement(cmd)
    }
}

impl From<NetworkManagement> for Command {
    fn from(cmd: NetworkManagement) -> Self {
        Self::NetworkManagement(cmd)
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::DeviceAndServiceDiscovery(cmd) => match cmd {
                DeviceAndServiceDiscovery::NwkAddrReq(cmd) => Iter::NwkAddrReq(cmd.to_le_stream()),
                DeviceAndServiceDiscovery::IeeeAddrReq(cmd) => {
                    Iter::IeeeAddrReq(cmd.to_le_stream())
                }
                DeviceAndServiceDiscovery::NodeDescReq(cmd) => {
                    Iter::NodeDescReq(cmd.to_le_stream())
                }
                DeviceAndServiceDiscovery::NodeDescRsp(cmd) => {
                    Iter::NodeDescRsp(cmd.to_le_stream().into())
                }
                DeviceAndServiceDiscovery::PowerDescReq(cmd) => {
                    Iter::PowerDescReq(cmd.to_le_stream())
                }
                DeviceAndServiceDiscovery::SimpleDescReq(cmd) => {
                    Iter::SimpleDescReq(cmd.to_le_stream())
                }
                DeviceAndServiceDiscovery::SimpleDescRsp(cmd) => {
                    Iter::SimpleDescRsp(cmd.to_le_stream().into())
                }
                DeviceAndServiceDiscovery::ActiveEpReq(cmd) => {
                    Iter::ActiveEpReq(cmd.to_le_stream())
                }
                DeviceAndServiceDiscovery::ActiveEpRsp(cmd) => {
                    Iter::ActiveEpRsp(cmd.to_le_stream())
                }
                DeviceAndServiceDiscovery::MatchDescReq(cmd) => {
                    Iter::MatchDescReq(cmd.to_le_stream().into())
                }
                DeviceAndServiceDiscovery::MatchDescRsp(cmd) => {
                    Iter::MatchDescRsp(cmd.to_le_stream())
                }
                DeviceAndServiceDiscovery::DeviceAnnce(cmd) => {
                    Iter::DeviceAnnce(cmd.to_le_stream())
                }
                DeviceAndServiceDiscovery::ParentAnnce(cmd) => {
                    Iter::ParentAnnce(cmd.to_le_stream().into())
                }
                DeviceAndServiceDiscovery::SystemServerDiscoveryReq(cmd) => {
                    Iter::SystemServerDiscoveryReq(cmd.to_le_stream())
                }
            },
            Self::BindManagement(cmd) => match cmd {
                BindManagement::BindReq(cmd) => Iter::BindReq(cmd.to_le_stream()),
                BindManagement::BindRsp(cmd) => Iter::BindRsp(cmd.to_le_stream()),
                BindManagement::UnbindReq(cmd) => Iter::UnbindReq(cmd.to_le_stream()),
                BindManagement::ClearAllBindingsReq(cmd) => {
                    Iter::ClearAllBindingsReq(cmd.to_le_stream())
                }
            },
            Self::NetworkManagement(cmd) => match cmd {
                NetworkManagement::MgmtLqiReq(cmd) => Iter::MgmtLqiReq(cmd.to_le_stream()),
                NetworkManagement::MgmtRtgReq(cmd) => Iter::MgmtRtgReq(cmd.to_le_stream()),
                NetworkManagement::MgmtBindReq(cmd) => Iter::MgmtBindReq(cmd.to_le_stream()),
                NetworkManagement::MgmtLeaveReq(cmd) => Iter::MgmtLeaveReq(cmd.to_le_stream()),
                NetworkManagement::MgmtPermitJoiningReq(cmd) => {
                    Iter::MgmtPermitJoiningReq(cmd.to_le_stream())
                }
                NetworkManagement::MgmtNwkUpdateReq(cmd) => {
                    Iter::MgmtNwkUpdateReq(cmd.to_le_stream())
                }
                NetworkManagement::MgmtNwkEnhancedUpdateReq(cmd) => {
                    Iter::MgmtNwkEnhancedUpdateReq(cmd.to_le_stream().into())
                }
                NetworkManagement::MgmtNwkIeeeJoiningListReq(cmd) => {
                    Iter::MgmtNwkIeeeJoiningListReq(cmd.to_le_stream())
                }
                NetworkManagement::MgmtNwkBeaconSurveyReq(cmd) => {
                    Iter::MgmtNwkBeaconSurveyReq(cmd.to_le_stream())
                }
                NetworkManagement::MgmtPermitJoiningRsp(cmd) => {
                    Iter::MgmtPermitJoiningRsp(cmd.to_le_stream())
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum Iter {
    NwkAddrReq(<NwkAddrReq as ToLeStream>::Iter),
    IeeeAddrReq(<IeeeAddrReq as ToLeStream>::Iter),
    NodeDescReq(<NodeDescReq as ToLeStream>::Iter),
    NodeDescRsp(Box<<NodeDescRsp as ToLeStream>::Iter>),
    PowerDescReq(<PowerDescReq as ToLeStream>::Iter),
    SimpleDescReq(<SimpleDescReq as ToLeStream>::Iter),
    SimpleDescRsp(Box<<SimpleDescRsp as ToLeStream>::Iter>),
    ActiveEpReq(<ActiveEpReq as ToLeStream>::Iter),
    ActiveEpRsp(<ActiveEpRsp as ToLeStream>::Iter),
    MatchDescReq(Box<<MatchDescReq as ToLeStream>::Iter>),
    MatchDescRsp(<MatchDescRsp as ToLeStream>::Iter),
    DeviceAnnce(<DeviceAnnce as ToLeStream>::Iter),
    ParentAnnce(Box<<ParentAnnce as ToLeStream>::Iter>),
    SystemServerDiscoveryReq(<SystemServerDiscoveryReq as ToLeStream>::Iter),
    BindReq(<BindReq as ToLeStream>::Iter),
    BindRsp(<BindRsp as ToLeStream>::Iter),
    UnbindReq(<UnbindReq as ToLeStream>::Iter),
    ClearAllBindingsReq(<ClearAllBindingsReq as ToLeStream>::Iter),
    MgmtLqiReq(<MgmtLqiReq as ToLeStream>::Iter),
    MgmtRtgReq(<MgmtRtgReq as ToLeStream>::Iter),
    MgmtBindReq(<MgmtBindReq as ToLeStream>::Iter),
    MgmtLeaveReq(<MgmtLeaveReq as ToLeStream>::Iter),
    MgmtPermitJoiningReq(<MgmtPermitJoiningReq as ToLeStream>::Iter),
    MgmtNwkUpdateReq(<MgmtNwkUpdateReq as ToLeStream>::Iter),
    MgmtNwkEnhancedUpdateReq(Box<<MgmtNwkEnhancedUpdateReq as ToLeStream>::Iter>),
    MgmtNwkIeeeJoiningListReq(<MgmtNwkIeeeJoiningListReq as ToLeStream>::Iter),
    MgmtNwkBeaconSurveyReq(<MgmtNwkBeaconSurveyReq as ToLeStream>::Iter),
    MgmtPermitJoiningRsp(<MgmtPermitJoiningRsp as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::NwkAddrReq(iter) => iter.next(),
            Self::IeeeAddrReq(iter) => iter.next(),
            Self::NodeDescReq(iter) => iter.next(),
            Self::NodeDescRsp(iter) => iter.next(),
            Self::PowerDescReq(iter) => iter.next(),
            Self::SimpleDescReq(iter) => iter.next(),
            Self::SimpleDescRsp(iter) => iter.next(),
            Self::ActiveEpReq(iter) => iter.next(),
            Self::ActiveEpRsp(iter) => iter.next(),
            Self::MatchDescReq(iter) => iter.next(),
            Self::MatchDescRsp(iter) => iter.next(),
            Self::DeviceAnnce(iter) => iter.next(),
            Self::ParentAnnce(iter) => iter.next(),
            Self::SystemServerDiscoveryReq(iter) => iter.next(),
            Self::BindReq(iter) => iter.next(),
            Self::BindRsp(iter) => iter.next(),
            Self::UnbindReq(iter) => iter.next(),
            Self::ClearAllBindingsReq(iter) => iter.next(),
            Self::MgmtLqiReq(iter) => iter.next(),
            Self::MgmtRtgReq(iter) => iter.next(),
            Self::MgmtBindReq(iter) => iter.next(),
            Self::MgmtLeaveReq(iter) => iter.next(),
            Self::MgmtPermitJoiningReq(iter) => iter.next(),
            Self::MgmtNwkUpdateReq(iter) => iter.next(),
            Self::MgmtNwkEnhancedUpdateReq(iter) => iter.next(),
            Self::MgmtNwkIeeeJoiningListReq(iter) => iter.next(),
            Self::MgmtNwkBeaconSurveyReq(iter) => iter.next(),
            Self::MgmtPermitJoiningRsp(iter) => iter.next(),
        }
    }
}
