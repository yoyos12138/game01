use bevy::ecs::component::Component;


//移动触发每秒
#[derive(Debug, Component)]pub struct Speed(pub f32);

//
#[derive(Debug, Component)]pub struct Hp(pub f32);
