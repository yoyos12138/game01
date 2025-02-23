use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Startup, Update},
    render::texture::ImagePlugin,
    time::{Timer, TimerMode},
};
use resource::{UserMoveTimer, UserRadius, UserSpeed};
use system::{add_camera2d, add_user, limit_user_in_windows, move_user, rotate_user, init_window};

pub mod entity;
pub mod plugin;
pub mod resource;
pub mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup,init_window)
        .add_systems(Startup, add_camera2d)
        .insert_resource(UserRadius(35.0))
        .insert_resource(UserSpeed(5.0))
        .add_systems(Startup, add_user)
        .insert_resource(UserMoveTimer(Timer::from_seconds(
            0.01,
            TimerMode::Repeating,
        )))
        .add_systems(Update, move_user)
        .add_systems(Update, limit_user_in_windows)
        .add_systems(Update, rotate_user)
        .run();
}
