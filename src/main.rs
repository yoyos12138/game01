pub mod components;
pub mod init;
pub mod player;

use crate::init::plugins::InitPlugin;
use bevy::{DefaultPlugins, app::App};
use player::plugins::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InitPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
