use zcl::lighting::color_control::MoveToColor;

use crate::proxies::EndpointProxy;
use crate::{Error, Proxy};

/// Trait for Color Control cluster operations.
pub trait ColorControl {
    /// Move to the specified color (x, y) over the given transition time.
    fn move_to_color(
        &self,
        color_x: u16,
        color_y: u16,
        transition_time: u16,
    ) -> impl Future<Output = Result<u8, Error>> + Send;
}

impl<T> ColorControl for EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    async fn move_to_color(
        &self,
        color_x: u16,
        color_y: u16,
        transition_time: u16,
    ) -> Result<u8, Error> {
        self.zcl()
            .unicast(MoveToColor::new(color_x, color_y, transition_time, 0, 0))
            .await
    }
}
