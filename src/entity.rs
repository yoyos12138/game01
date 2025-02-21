use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}