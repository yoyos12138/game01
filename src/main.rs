pub mod entity;
pub mod system;

use bevy::{app::{App, Startup, Update}, DefaultPlugins};
use system::{add_persons, first_system, greet_peoples, update_people};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,(
            add_persons,
            first_system
    ))
    .add_systems(Update,(
        greet_peoples,
        update_people
    ))
    .run();
}