pub mod entity;
pub mod plugin;
pub mod resource;
pub mod system;

use bevy::{DefaultPlugins, app::App};
use plugin::{MyFirstPLugin, ResTestPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MyFirstPLugin)
        .add_plugins(ResTestPlugin)
        .run();
}
