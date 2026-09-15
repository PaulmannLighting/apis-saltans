use zb_core::types::{Type, Uint8};
use zb_core::{Cluster, Profile, Profiled};

use crate::macros::zcl_attributes;

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

#[test]
fn generated_id_display_and_parsing_round_trip() {
    for id in [
        Id::ReadOnly,
        Id::Writable,
        Id::ClusterRevision,
        Id::AttributeReportingStatus,
    ] {
        assert_eq!(id.to_string().parse(), Ok(id));
    }
}

mod required_cluster {
    use super::{Cluster, Profile, Profiled, Uint8};
    use crate::macros::zcl_attributes;

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
