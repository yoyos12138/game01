use bevy::ecs::component::Component;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}