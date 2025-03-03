use bevy::ecs::component::Component;

//主摄像头
#[derive(Component)]
pub struct MajorCamera;

//玩家
#[derive(Component)]
pub struct User;

//生命值
#[derive(Component)]
pub struct Life(pub f64);

//子弹
#[derive(Component)]
pub struct Bullet;
