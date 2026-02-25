use le_stream::ToLeStream;
use macaddr::MacAddr8;
use zdp::{BindReq, Destination, Frame, Service, UnbindReq};
use zigbee::{Cluster, Endpoint};

use crate::{Binding, Error, Proxy};

/// A proxy structure to interact with ZDP commands on a specific endpoint.
#[derive(Clone, Debug)]
pub struct ZdpProxy<'proxy, T> {
    proxy: &'proxy T,
    pan_id: u16,
    endpoint: Endpoint,
}

impl<'proxy, T> ZdpProxy<'proxy, T> {
    /// Create a new `ZdpProxy`.
    pub(crate) const fn new(proxy: &'proxy T, pan_id: u16, endpoint: Endpoint) -> Self {
        Self {
            proxy,
            pan_id,
            endpoint,
        }
    }
}

impl<T> ZdpProxy<'_, T>
where
    T: Proxy + Sync,
{
    /// Send a ZDP command to a specific endpoint on a device.
    pub async fn unicast<C>(&self, command: C) -> Result<u8, Error>
    where
        C: Cluster + Service + ToLeStream,
    {
        self.proxy
            .unicast(
                self.pan_id,
                self.endpoint,
                Frame::new(self.proxy.next_transaction_seq().await?, command).into(),
            )
            .await
    }
}

impl<T> Binding for ZdpProxy<'_, T>
where
    T: Proxy + Sync,
{
    async fn bind(
        &self,
        src_address: MacAddr8,
        src_endpoint: Endpoint,
        cluster_id: u16,
        destination: Destination,
    ) -> Result<u8, Error> {
        self.unicast(BindReq::new(
            src_address,
            src_endpoint.into(),
            cluster_id,
            destination,
        ))
        .await
    }

    async fn unbind(
        &self,
        src_address: MacAddr8,
        src_endpoint: Endpoint,
        cluster_id: u16,
        destination: Destination,
    ) -> Result<u8, Error> {
        self.unicast(UnbindReq::new(
            src_address,
            src_endpoint.into(),
            cluster_id,
            destination,
        ))
        .await
    }
}
