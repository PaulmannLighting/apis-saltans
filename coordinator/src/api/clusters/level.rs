use zb_aps::apsde::IndividualEndpoint;
use zb_core::Destination;
use zb_core::units::{Deciseconds, UnitsPerSecond};
use zb_zcl::Options;
use zb_zcl::level::{
    Mode, Move, MoveToClosestFrequency, MoveToLevel, MoveToLevelWithOnOff, MoveWithOnOff, Step,
    StepWithOnOff, Stop, StopWithOnOff,
};

use crate::Error;
use crate::api::Zcl;
use crate::api::zcl::request_without_response;

/// Trait for the Level cluster.
///
/// Each method requires the local APS source endpoint, disables ZCL Default Responses, and awaits
/// the acknowledged APS transmission before returning.
pub trait Level {
    /// Move to level command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn move_to_level(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        level: u8,
        transition_time: Deciseconds,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn r#move(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        mode: Mode<UnitsPerSecond>,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Step command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn step(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        mode: Mode<u8>,
        transition_time: Deciseconds,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Stop command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn stop(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move to level with on/off command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn move_to_level_with_on_off(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        level: u8,
        transition_time: Deciseconds,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move with on/off command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn move_with_on_off(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        mode: Mode<UnitsPerSecond>,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Step with on/off command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn step_with_on_off(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        mode: Mode<u8>,
        transition_time: Deciseconds,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Stop with on/off command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn stop_with_on_off(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        options: Options,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Move to the closest frequency command.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the command cannot be queued or transmitted.
    fn move_to_closest_frequency(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        frequency: u16,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl<T> Level for T
where
    T: Zcl + Sync,
{
    async fn move_to_level(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        level: u8,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            MoveToLevel::new(level, transition_time, options),
        ))
        .await
    }

    async fn r#move(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        mode: Mode<UnitsPerSecond>,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            Move::new(mode, options),
        ))
        .await
    }

    async fn step(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        mode: Mode<u8>,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            Step::new(mode, transition_time, options),
        ))
        .await
    }

    async fn stop(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            Stop::new(options),
        ))
        .await
    }

    async fn move_to_level_with_on_off(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        level: u8,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            MoveToLevelWithOnOff::new(level, transition_time, options),
        ))
        .await
    }

    async fn move_with_on_off(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        mode: Mode<UnitsPerSecond>,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            MoveWithOnOff::new(mode, options),
        ))
        .await
    }

    async fn step_with_on_off(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        mode: Mode<u8>,
        transition_time: Deciseconds,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            StepWithOnOff::new(mode, transition_time, options),
        ))
        .await
    }

    async fn stop_with_on_off(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        options: Options,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            StopWithOnOff::new(options),
        ))
        .await
    }

    async fn move_to_closest_frequency(
        &self,
        destination: Destination,
        source_endpoint: IndividualEndpoint,
        frequency: u16,
    ) -> Result<(), Error> {
        self.transmit(request_without_response(
            destination,
            source_endpoint,
            MoveToClosestFrequency::new(frequency),
        ))
        .await
    }
}
