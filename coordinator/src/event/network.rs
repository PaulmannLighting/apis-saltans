use serde::{Deserialize, Serialize};
use zb_hw::RouteError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Network {
    Up,
    Down,
    Opened,
    Closed,
    Error(Error),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    Route(RouteError),
}
