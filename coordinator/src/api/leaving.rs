use zb_core::IeeeAddress;
use zb_core::short_id::Device;
use zb_zdp::{LeaveReqFlags, MgmtLeaveReq};

use crate::{Error, StatusExt, Zdp};

/// Trait for requesting that devices leave the network.
pub trait Leaving {
    /// Request that a device identified by its NWK short ID leave the network.
    ///
    /// The request is sent directly to `device`, so its device-address payload is null. `None`
    /// sends an empty flag set; pass [`LeaveReqFlags`] to request rejoining, child removal, or both.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request cannot be queued, transmission or reception fails, the
    /// response is invalid, or it completes with a non-success ZDP status.
    fn leave(
        &self,
        device: Device,
        flags: Option<LeaveReqFlags>,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Leaving for T
where
    T: Zdp + Sync,
{
    async fn leave(&self, device: Device, flags: Option<LeaveReqFlags>) -> Result<(), Error> {
        self.communicate(
            device,
            MgmtLeaveReq::new(
                IeeeAddress::default(),
                flags.unwrap_or_else(LeaveReqFlags::empty),
            ),
        )
        .await?
        .await?
        .status()
        .ensure_success()
    }
}

#[cfg(test)]
mod tests {
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use bytes::Bytes;
    use le_stream::FromLeStream;
    use tokio::sync::mpsc::channel;
    use zb_aps::apsde::{
        DataRequest, IndividualEndpoint, NetworkAddress, NetworkDestination, RequestDestination,
    };
    use zb_core::short_id::Device;
    use zb_core::{Endpoint, IeeeAddress};
    use zb_zdp::{LeaveReqFlags, MgmtLeaveReq};

    use super::Leaving;
    use crate::zdp::Message;

    const CHANNEL_SIZE: usize = 1;
    const SHORT_ID: u16 = 0x1234;
    const DEVICE: Device = match Device::new(SHORT_ID) {
        Some(device) => device,
        None => panic!("test short ID must be a device address"),
    };

    #[test]
    fn leave_without_flags_sends_a_request_to_the_device_itself() {
        let (device, request) = queued_leave_request(None);
        let endpoint = IndividualEndpoint::new(Endpoint::Data).expect("ZDO endpoint is individual");
        let destination: RequestDestination = NetworkDestination::new(
            NetworkAddress::new(SHORT_ID).expect("test short ID is a network address"),
            endpoint,
        )
        .into();
        let payload = MgmtLeaveReq::from_le_stream(request.asdu().iter().copied())
            .expect("leave request payload must parse");

        assert_eq!(device, DEVICE);
        assert_eq!(request.destination(), destination);
        assert_eq!(request.cluster_id(), MgmtLeaveReq::ID);
        assert_eq!(payload.device_address(), IeeeAddress::default());
        assert_eq!(payload.flags(), LeaveReqFlags::empty());
    }

    #[test]
    fn leave_sends_the_requested_flags() {
        let flags = LeaveReqFlags::REJOIN | LeaveReqFlags::REMOVE_CHILDREN;
        let (_, request) = queued_leave_request(Some(flags));
        let payload = MgmtLeaveReq::from_le_stream(request.asdu().iter().copied())
            .expect("leave request payload must parse");

        assert_eq!(payload.flags(), flags);
    }

    fn queued_leave_request(flags: Option<LeaveReqFlags>) -> (Device, DataRequest<Bytes>) {
        let (sender, mut messages) = channel(CHANNEL_SIZE);
        let mut leave = pin!(sender.leave(DEVICE, flags));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(leave.as_mut().poll(&mut context), Poll::Pending));

        let Message::Communicate {
            device,
            request,
            response: _,
        } = messages
            .try_recv()
            .expect("leave request must be queued for the ZDP actor")
        else {
            panic!("expected ZDP communication request");
        };

        (device, request)
    }
}
