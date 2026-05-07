use zcl::global::read_attributes;
use zcl::{Cluster, Customizable, Global, HeaderFactory, global};
use zigbee::Endpoint;

use crate::demux::Subscribe;
use crate::{Command, Error, Event, Proxy};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Transceiver<T, R> {
    tx_proxy: T,
    demux_proxy: R,
}

impl<T, R> Transceiver<T, R> {
    /// Create a new transceiver.
    #[must_use]
    pub const fn new(tx_proxy: T, demux_proxy: R) -> Self {
        Self {
            tx_proxy,
            demux_proxy,
        }
    }
}

impl<T, R> Transceiver<T, R>
where
    T: Proxy,
    R: Subscribe,
{
    async fn read_attributes(
        &self,
        pan_id: u16,
        endpoint: Endpoint,
        cluster_id: u16,
        attribute_ids: Box<[u16]>,
        manufacturer_code: Option<u16>,
    ) -> Result<read_attributes::Response, Error> {
        let seq = self.tx_proxy.next_transaction_seq().await?;
        let response = self.demux_proxy.subscribe(seq).await?;
        self.tx_proxy
            .unicast(
                pan_id,
                endpoint,
                read_attributes::Command::new(attribute_ids)
                    .for_cluster(cluster_id)
                    .with_manufacturer_code(manufacturer_code)
                    .frame(seq)
                    .into(),
            )
            .await?;

        if let Event::MessageReceived {
            src_address,
            aps_frame,
        } = response.await?
        {
            if src_address != pan_id {
                todo!("Handle unexpected source address.")
            }

            let (aps_header, payload) = aps_frame.into_parts();

            if let Command::Zcl(zcl_frame) = payload {
                let (zcl_header, payload) = zcl_frame.into_parts();

                if let Cluster::Global(global::Command::ReadAttributesResponse(response)) = payload
                {
                    return Ok(response);
                } else {
                    todo!("Handle unexpected payload.")
                }
            } else {
                todo!("Handle unexpected payload.")
            }
        } else {
            todo!("Handle unexpected response.")
        }
    }
}
