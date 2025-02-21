use bevy::{
    app::{Plugin, Startup, Update},
    time::{Timer, TimerMode},
};

use crate::{
    resource::GameTimer,
    system::{add_persons, first_system, greet_peoples, greet_with_time, update_people},
};

pub struct MyFirstPLugin;
impl Plugin for MyFirstPLugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, (add_persons, first_system))
            .add_systems(Update, (greet_peoples, update_people));
    }
}

pub struct ResTestPlugin;
impl Plugin for ResTestPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, greet_with_time);
    }
}
