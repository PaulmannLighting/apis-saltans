use zb_core::types::{Type, Uint8};
use zb_core::{Cluster, Profile, Profiled};

use crate::macros::zcl_attributes;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Custom(Uint8);

impl zb_core::TypeId for Custom {
    const ID: u8 = <Uint8 as zb_core::TypeId>::ID;
}

impl From<Custom> for Type {
    fn from(value: Custom) -> Self {
        value.0.into()
    }
}

impl TryFrom<Type> for Custom {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        Uint8::try_from(value).map(Self)
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

#[test]
fn optional_reporting_preserves_read_and_write_permissions() {
    const WRITE_ONLY_ID: u16 = 0x0002;
    const VALUE: Uint8 = Uint8::new(7);

    assert_eq!(Id::try_from(WRITE_ONLY_ID), Err(WRITE_ONLY_ID));
    assert_eq!(
        Reportable::try_from((WRITE_ONLY_ID, Type::Uint8(VALUE))),
        Ok(Reportable::WriteOnly(Custom(VALUE)))
    );
    // An exhaustive match ensures reporting did not introduce a read-only writable variant.
    let writable = Writable::WriteOnly(Custom(VALUE));
    match writable {
        Writable::Writable(_) | Writable::WriteOnly(_) => {}
    }
}

#[test]
fn optional_reporting_encodes_analog_discrete_and_global_attributes() {
    use le_stream::ToLeStream;
    use zb_core::types::{Int32, String, Uint16};

    use crate::global::configure_reporting::send::AttributeReportingConfiguration;
    use crate::{Analog, Discrete, basic, level, time};

    const MINIMUM: u16 = 10;
    const MAXIMUM: u16 = 60;
    const CHANGE: Uint16 = Uint16::new(2);
    const TIME_CHANGE: Int32 = Int32::new(2);

    let remaining_time: AttributeReportingConfiguration =
        level::SendReport::RemainingTime(Analog::new(MINIMUM, MAXIMUM, CHANGE)).into();
    assert_eq!(
        remaining_time.to_le_stream().collect::<Vec<_>>(),
        [0x00, 0x01, 0x00, 0x21, 10, 0, 60, 0, 2, 0]
    );
    let manufacturer: AttributeReportingConfiguration =
        basic::SendReport::ManufacturerName(Discrete::<String<32>>::new(MINIMUM, MAXIMUM)).into();
    assert_eq!(
        manufacturer.to_le_stream().collect::<Vec<_>>(),
        [0x00, 0x04, 0x00, 0x42, 10, 0, 60, 0]
    );
    let revision: AttributeReportingConfiguration =
        basic::SendReport::ClusterRevision(Analog::new(MINIMUM, MAXIMUM, CHANGE)).into();
    assert_eq!(
        revision.to_le_stream().collect::<Vec<_>>(),
        [0x00, 0xfd, 0xff, 0x21, 10, 0, 60, 0, 2, 0]
    );
    let timezone: AttributeReportingConfiguration =
        time::SendReport::TimeZone(Analog::new(MINIMUM, MAXIMUM, TIME_CHANGE)).into();
    assert_eq!(
        timezone.to_le_stream().collect::<Vec<_>>(),
        [0x00, 0x02, 0x00, 0x2b, 10, 0, 60, 0, 2, 0, 0, 0]
    );
}
