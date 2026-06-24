use zcl::Options;
use zcl::general::level::{
    Mode, Move, MoveToClosestFrequency, MoveToLevel, MoveToLevelWithOnOff, MoveWithOnOff, Step,
    StepWithOnOff, Stop, StopWithOnOff,
};
use zigbee::types::Uint16;

use crate::transceiver::zcl::Handle;
use crate::{Coordinator, Destination, Error};

/// Trait for the Level cluster.
pub trait Level {
    /// Move to level command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_level(
        &self,
        destination: Destination,
        level: u8,
        transition_time: Uint16,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn r#move(
        &self,
        destination: Destination,
        mode: Mode,
        rate: u8,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Step command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn step(
        &self,
        destination: Destination,
        mode: Mode,
        size: u8,
        transition_time: u16,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Stop command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn stop(
        &self,
        destination: Destination,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move to level with on/off command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_level_with_on_off(
        &self,
        destination: Destination,
        level: u8,
        transition_time: Uint16,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move with on/off command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_with_on_off(
        &self,
        destination: Destination,
        mode: Mode,
        rate: u8,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Step with on/off command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn step_with_on_off(
        &self,
        destination: Destination,
        mode: Mode,
        size: u8,
        transition_time: u16,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Stop with on/off command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn stop_with_on_off(
        &self,
        destination: Destination,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move to the closest frequency command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if execution of the command failed.
    fn move_to_closest_frequency(
        &self,
        destination: Destination,
        frequency: u16,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Level for Coordinator {
    async fn move_to_level(
        &self,
        destination: Destination,
        level: u8,
        transition_time: Uint16,
        options: Options,
    ) -> Result<(), Error> {
        self.send_static_cluster(
            destination,
            MoveToLevel::new(level, transition_time, options),
        )
        .await
    }

    async fn r#move(
        &self,
        destination: Destination,
        mode: Mode,
        rate: u8,
        options: Options,
    ) -> Result<(), Error> {
        self.send_static_cluster(destination, Move::new(mode, rate, options))
            .await
    }

    async fn step(
        &self,
        destination: Destination,
        mode: Mode,
        size: u8,
        transition_time: u16,
        options: Options,
    ) -> Result<(), Error> {
        self.send_static_cluster(destination, Step::new(mode, size, transition_time, options))
            .await
    }

    async fn stop(&self, destination: Destination, options: Options) -> Result<(), Error> {
        self.send_static_cluster(destination, Stop::new(options))
            .await
    }

    async fn move_to_level_with_on_off(
        &self,
        destination: Destination,
        level: u8,
        transition_time: Uint16,
        options: Options,
    ) -> Result<(), Error> {
        self.send_static_cluster(
            destination,
            MoveToLevelWithOnOff::new(level, transition_time, options),
        )
        .await
    }

    async fn move_with_on_off(
        &self,
        destination: Destination,
        mode: Mode,
        rate: u8,
        options: Options,
    ) -> Result<(), Error> {
        self.send_static_cluster(destination, MoveWithOnOff::new(mode, rate, options))
            .await
    }

    async fn step_with_on_off(
        &self,
        destination: Destination,
        mode: Mode,
        size: u8,
        transition_time: u16,
        options: Options,
    ) -> Result<(), Error> {
        self.send_static_cluster(
            destination,
            StepWithOnOff::new(mode, size, transition_time, options),
        )
        .await
    }

    async fn stop_with_on_off(
        &self,
        destination: Destination,
        options: Options,
    ) -> Result<(), Error> {
        self.send_static_cluster(destination, StopWithOnOff::new(options))
            .await
    }

    async fn move_to_closest_frequency(
        &self,
        destination: Destination,
        frequency: u16,
    ) -> Result<(), Error> {
        self.send_static_cluster(destination, MoveToClosestFrequency::new(frequency))
            .await
    }
}
