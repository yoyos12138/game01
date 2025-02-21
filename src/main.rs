pub mod entity;
pub mod system;

use bevy::app::{App, Startup};
use system::first_system;

fn main() {
    App::new()
    .add_systems(Startup, first_system)
    .run();
}