use crate::zcl::cluster::Cluster;
use std::time::{Duration, SystemTime};

const DEFAULT_MINIMUM_REPORTING_PERIOD: Duration = Duration::from_secs(0xffff);
const DEFAULT_MAXIMUM_REPORTING_PERIOD: Duration = Duration::from_secs(0xffff);
const DEFAULT_MAXIMUM_REPORTING_TIMEOUT: Duration = Duration::from_secs(0xffff);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute<T> {
    cluster: Cluster,
    name: String,
    data_type: DataType,
    mandatory: bool,
    implemented: bool,
    readable: bool,
    writable: bool,
    reportable: bool,
    minimum_reporting_period: Duration,
    maximum_reporting_period: Duration,
    reporting_change: T,
    reporting_timeout: Duration,
    manufacturer_code: u16,
    last_report_time: SystemTime,
    last_value: T,
}
