//! Commands for the On/Off cluster.

use le_stream::ToLeStream;
use apis_saltans_core::{ClusterId, Cluster, Direction};
use apis_saltans_macros::ParseZclFrame;

pub use self::off::Off;
pub use self::off_with_effect::{DelayedAllOff, DyingLight, Effect, OffWithEffect};
pub use self::on::On;
pub use self::on_with_recall_global_scene::OnWithRecallGlobalScene;
pub use self::on_with_timed_off::{OnOffControl, OnWithTimedOff};
pub use self::toggle::Toggle;
use crate::{CommandDispatch, Scope};

mod off;
mod off_with_effect;
mod on;
mod on_with_recall_global_scene;
mod on_with_timed_off;
mod toggle;

/// Available On/Off cluster commands.
#[cfg_attr(target_pointer_width = "64", expect(variant_size_differences))]
#[derive(Clone, Debug, Eq, PartialEq, Hash, ParseZclFrame)]
pub enum Command {
    /// Off command.
    Off(Off),

    /// On command.
    On(On),

    /// Toggle command.
    Toggle(Toggle),

    /// Off with Effect command.
    OffWithEffect(OffWithEffect),

    /// On with Recall Global Scene command.
    OnWithRecallGlobalScene(OnWithRecallGlobalScene),

    /// On with Timed Off command.
    OnWithTimedOff(OnWithTimedOff),
}

impl Cluster<ClusterId> for Command {
    const ID: ClusterId = ClusterId::OnOff;
}

impl From<Command> for crate::Cluster {
    fn from(command: Command) -> Self {
        Self::OnOff(command)
    }
}

impl From<Off> for Command {
    fn from(command: Off) -> Self {
        Self::Off(command)
    }
}

impl From<On> for Command {
    fn from(command: On) -> Self {
        Self::On(command)
    }
}

impl From<Toggle> for Command {
    fn from(command: Toggle) -> Self {
        Self::Toggle(command)
    }
}

impl From<OffWithEffect> for Command {
    fn from(command: OffWithEffect) -> Self {
        Self::OffWithEffect(command)
    }
}

impl From<OnWithRecallGlobalScene> for Command {
    fn from(command: OnWithRecallGlobalScene) -> Self {
        Self::OnWithRecallGlobalScene(command)
    }
}

impl From<OnWithTimedOff> for Command {
    fn from(command: OnWithTimedOff) -> Self {
        Self::OnWithTimedOff(command)
    }
}

impl CommandDispatch for Command {
    fn command_id(&self) -> u8 {
        match self {
            Self::Off(cmd) => cmd.command_id(),
            Self::On(cmd) => cmd.command_id(),
            Self::Toggle(cmd) => cmd.command_id(),
            Self::OffWithEffect(cmd) => cmd.command_id(),
            Self::OnWithRecallGlobalScene(cmd) => cmd.command_id(),
            Self::OnWithTimedOff(cmd) => cmd.command_id(),
        }
    }

    fn scope(&self) -> Scope {
        match self {
            Self::Off(cmd) => cmd.scope(),
            Self::On(cmd) => cmd.scope(),
            Self::Toggle(cmd) => cmd.scope(),
            Self::OffWithEffect(cmd) => cmd.scope(),
            Self::OnWithRecallGlobalScene(cmd) => cmd.scope(),
            Self::OnWithTimedOff(cmd) => cmd.scope(),
        }
    }

    fn direction(&self) -> Direction {
        match self {
            Self::Off(cmd) => cmd.direction(),
            Self::On(cmd) => cmd.direction(),
            Self::Toggle(cmd) => cmd.direction(),
            Self::OffWithEffect(cmd) => cmd.direction(),
            Self::OnWithRecallGlobalScene(cmd) => cmd.direction(),
            Self::OnWithTimedOff(cmd) => cmd.direction(),
        }
    }

    fn disable_default_response(&self) -> bool {
        match self {
            Self::Off(cmd) => cmd.disable_default_response(),
            Self::On(cmd) => cmd.disable_default_response(),
            Self::Toggle(cmd) => cmd.disable_default_response(),
            Self::OffWithEffect(cmd) => cmd.disable_default_response(),
            Self::OnWithRecallGlobalScene(cmd) => cmd.disable_default_response(),
            Self::OnWithTimedOff(cmd) => cmd.disable_default_response(),
        }
    }
}

impl ToLeStream for Command {
    type Iter = Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Off(cmd) => Iter::Off(cmd.to_le_stream()),
            Self::On(cmd) => Iter::On(cmd.to_le_stream()),
            Self::Toggle(cmd) => Iter::Toggle(cmd.to_le_stream()),
            Self::OffWithEffect(cmd) => Iter::OffWithEffect(cmd.to_le_stream()),
            Self::OnWithRecallGlobalScene(cmd) => Iter::OnWithRecallGlobalScene(cmd.to_le_stream()),
            Self::OnWithTimedOff(cmd) => Iter::OnWithTimedOff(cmd.to_le_stream()),
        }
    }
}

#[derive(Debug)]
pub enum Iter {
    Off(<Off as ToLeStream>::Iter),
    On(<On as ToLeStream>::Iter),
    Toggle(<Toggle as ToLeStream>::Iter),
    OffWithEffect(<OffWithEffect as ToLeStream>::Iter),
    OnWithRecallGlobalScene(<OnWithRecallGlobalScene as ToLeStream>::Iter),
    OnWithTimedOff(<OnWithTimedOff as ToLeStream>::Iter),
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::Off(iter) => iter.next(),
            Self::On(iter) => iter.next(),
            Self::Toggle(iter) => iter.next(),
            Self::OffWithEffect(iter) => iter.next(),
            Self::OnWithRecallGlobalScene(iter) => iter.next(),
            Self::OnWithTimedOff(iter) => iter.next(),
        }
    }
}
