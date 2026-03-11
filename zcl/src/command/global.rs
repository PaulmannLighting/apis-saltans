use le_stream::ToLeStream;
use zigbee::{ClusterId, Direction};

use crate::{Command, Customizable, Header, HeaderFactory, Scope};

/// Trait to mark global commands.
pub trait Global: Sized {
    /// Target the global command towards a cluster.
    fn for_cluster(self, cluster_id: u16) -> ClusterTargeted<Self> {
        ClusterTargeted {
            cluster_id,
            payload: self,
        }
    }
}

#[derive(Debug)]
pub struct ClusterTargeted<T> {
    cluster_id: u16,
    payload: T,
}

impl<T> ClusterId for ClusterTargeted<T> {
    fn cluster_id(&self) -> u16 {
        self.cluster_id
    }
}

impl<T> Command for ClusterTargeted<T>
where
    T: Command,
{
    const ID: u8 = T::ID;
    const DIRECTION: Direction = T::DIRECTION;
    const SCOPE: Scope = T::SCOPE;
    const DISABLE_DEFAULT_RESPONSE: bool = T::DISABLE_DEFAULT_RESPONSE;
}

#[expect(unsafe_code)]
// SAFETY: We delegate to another implementation of `HeaderFactory`.
unsafe impl<T> HeaderFactory for ClusterTargeted<T>
where
    T: HeaderFactory,
{
    fn header(&self, seq: u8) -> Header {
        self.payload.header(seq)
    }
}

impl<T> ToLeStream for ClusterTargeted<T>
where
    T: ToLeStream,
{
    type Iter = T::Iter;

    fn to_le_stream(self) -> Self::Iter {
        self.payload.to_le_stream()
    }
}

impl<T> Customizable for ClusterTargeted<T> where T: Customizable {}
