use zb_core::Destination;
use zb_core::units::{Deciseconds, UnitsPerSecond};
use zb_zcl::Options;
use zb_zcl::level::{
    Mode, Move, MoveToClosestFrequency, MoveToLevel, MoveToLevelWithOnOff, MoveWithOnOff, Step,
    StepWithOnOff, Stop, StopWithOnOff,
};

use crate::transceiver::zcl::Handle;
use crate::{Coordinator, Error};

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
        transition_time: Deciseconds,
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
        mode: Mode<UnitsPerSecond>,
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
        mode: Mode<u8>,
        transition_time: Deciseconds,
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
        transition_time: Deciseconds,
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
        mode: Mode<UnitsPerSecond>,
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
        mode: Mode<u8>,
        transition_time: Deciseconds,
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
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(
            destination,
            MoveToLevel::new(level, transition_time, options),
        )
        .await
    }

    async fn r#move(
        &self,
        destination: Destination,
        mode: Mode<UnitsPerSecond>,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(destination, Move::new(mode, options)).await
    }

    async fn step(
        &self,
        destination: Destination,
        mode: Mode<u8>,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(destination, Step::new(mode, transition_time, options))
            .await
    }

    async fn stop(&self, destination: Destination, options: Options) -> Result<(), Error> {
        self.transmit(destination, Stop::new(options)).await
    }

    async fn move_to_level_with_on_off(
        &self,
        destination: Destination,
        level: u8,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(
            destination,
            MoveToLevelWithOnOff::new(level, transition_time, options),
        )
        .await
    }

    async fn move_with_on_off(
        &self,
        destination: Destination,
        mode: Mode<UnitsPerSecond>,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(destination, MoveWithOnOff::new(mode, options))
            .await
    }

    async fn step_with_on_off(
        &self,
        destination: Destination,
        mode: Mode<u8>,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(
            destination,
            StepWithOnOff::new(mode, transition_time, options),
        )
        .await
    }

    async fn stop_with_on_off(
        &self,
        destination: Destination,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(destination, StopWithOnOff::new(options))
            .await
    }

    async fn move_to_closest_frequency(
        &self,
        destination: Destination,
        frequency: u16,
    ) -> Result<(), Error> {
        self.transmit(destination, MoveToClosestFrequency::new(frequency))
            .await
    }
}
