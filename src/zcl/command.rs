use crate::zcl::cluster::Cluster;

pub trait Command: Cluster {
    const ID: u8;
}
