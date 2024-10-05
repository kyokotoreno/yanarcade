//! Common types and utilities used throughout the game.

use bevy::{
    prelude::*,
    utils::Duration,
};
use lightyear::{
    prelude::*,
    shared::config,
};

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;
pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

pub fn build_shared_config() -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: SERVER_REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1. / FIXED_TIMESTEP_HZ),
        },
        mode: config::Mode::Separate,
    }
}

#[derive(Clone)]
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
    }
}

/// The role of a player in the game.
pub enum PlayerRole {
    /// The player is the senpai.
    Senpai,

    /// The player is a yandere, contains reference to their assigned senpai.
    Yandere(Entity),

    /// The player is a rival, contains reference to their assigned senpai.
    Rival(Entity),

    /// The player is a classmate.
    Classmate,

    // TODO: Bodyguard,
    // TODO: Detective,
}
