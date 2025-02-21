use bevy::{ecs::system::Resource, time::Timer};
//资源是全局唯一的单例数据，用于在系统之间共享信息。
#[derive(Resource)]
pub struct GameTimer(pub Timer);
