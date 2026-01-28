use bunt::Xy;
use zcl::Options;
use zcl::lighting::color_control::MoveToColor;

use crate::proxies::EndpointProxy;
use crate::{Error, Proxy};

/// Trait for Color Control cluster operations.
pub trait ColorControl {
    /// Move to the specified color (x, y) over the given transition time.
    fn move_to_xy(
        &self,
        color: Xy,
        transition_time: u16,
        options: Options,
    ) -> impl Future<Output = Result<u8, Error>> + Send;

    /// Move to the specified color (x, y) over the given transition time.
    fn move_to_color<T>(
        &self,
        color: T,
        transition_time: u16,
        options: Options,
    ) -> impl Future<Output = Result<u8, Error>> + Send
    where
        T: Into<Xy>,
    {
        self.move_to_xy(color.into(), transition_time, options)
    }
}

impl<T> ColorControl for EndpointProxy<'_, T>
where
    T: Proxy + Sync,
{
    async fn move_to_xy(
        &self,
        color: Xy,
        transition_time: u16,
        options: Options,
    ) -> Result<u8, Error> {
        self.zcl()
            .unicast(MoveToColor::new(
                color.x(),
                color.y(),
                transition_time,
                options,
            ))
            .await
    }
}
