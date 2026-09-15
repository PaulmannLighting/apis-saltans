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
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            num_enum::IntoPrimitive,
            num_enum::TryFromPrimitive,
            strum::Display,
            strum::EnumString,
        )]
        #[num_enum(error_type(name = u16, constructor = core::convert::identity))]
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
