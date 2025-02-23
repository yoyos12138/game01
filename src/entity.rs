use bevy::ecs::component::Component;

//主摄像头
#[derive(Component)]
pub struct MajorCamera;

//王将
#[derive(Component)]
pub struct User;

//生命值
#[derive(Component)]
pub struct Life(pub f64);
