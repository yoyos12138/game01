use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Startup, Update},
    math::Vec3,
    render::texture::ImagePlugin,
    time::{Timer, TimerMode},
};
use resource::{CursorPosition, UserMoveTimer, UserRadius, UserSpeed};
use system::{
    add_camera2d, add_user, init_window, limit_user_in_windows, move_user, rotate_user,
    set_curser_resource, spawn_bullet,
};

pub mod entity;
pub mod plugin;
pub mod resource;
pub mod system;
pub mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, init_window)
        .add_systems(Startup, add_camera2d)
        .insert_resource(UserRadius(35.0))
        .insert_resource(UserSpeed(6.0))
        .add_systems(Startup, add_user)
        .insert_resource(UserMoveTimer(Timer::from_seconds(
            0.01,
            TimerMode::Repeating,
        )))
        .add_systems(Update, move_user)
        .add_systems(Update, limit_user_in_windows)
        .add_systems(Update, rotate_user)
        .insert_resource(CursorPosition(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }))
        .add_systems(Update, set_curser_resource)
        .add_systems(Update, spawn_bullet)
        .run();
}
