use zb_aps::Data;
use zb_zcl::{AttributeReport, Cluster, Frame, global};

/// An event type.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Type {
    /// A cluster-specific command.
    Cluster(Cluster),

    /// An attribute report.
    AttributeReport(Box<[AttributeReport]>),
}

impl From<Data<Frame<Cluster>>> for Type {
    fn from(data: Data<Frame<Cluster>>) -> Self {
        let (aps_header, payload) = data.into_parts();
        let (_zcl_header, command) = payload.into_parts();

        if let Cluster::Global(global::Command::ReportAttributes(report_attributes)) = command {
            Self::AttributeReport(
                report_attributes
                    .into_reports()
                    .into_iter()
                    .filter_map(|report| {
                        AttributeReport::parse(
                            aps_header.cluster_id(),
                            report.attribute_id(),
                            report.into_data(),
                        )
                        .ok()
                    })
                    .collect(),
            )
        } else {
            Self::Cluster(command)
        }
    }
}
