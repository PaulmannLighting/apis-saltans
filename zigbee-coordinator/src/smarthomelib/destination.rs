use macaddr::MacAddr8;
use smarthomelib::Destination;
use zigbee::Application;

impl From<Destination<MacAddr8, Application, u16>> for crate::Destination {
    fn from(destination: Destination<MacAddr8, Application, u16>) -> Self {
        match destination {
            Destination::Device(device) => Self::Endpoint {
                ieee_address: device,
                endpoint: Application::default(),
            },
            Destination::Endpoint { device, endpoint } => Self::Endpoint {
                ieee_address: device,
                endpoint,
            },
            Destination::Group(group) => Self::Group(group),
        }
    }
}
