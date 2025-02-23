use bevy::{ecs::system::Resource, time::Timer};
//资源是全局唯一的单例数据，用于在系统之间共享信息。

//自定义触发器
#[derive(Resource)]
pub struct GameTimer(pub Timer);

//玩家移动触发器
#[derive(Resource)]
pub struct UserMoveTimer(pub Timer);

//玩家初始半径
#[derive(Resource, Default)]
pub struct UserRadius(pub f32);

//玩家速度
#[derive(Resource, Default)]
pub struct UserSpeed(pub f32);
