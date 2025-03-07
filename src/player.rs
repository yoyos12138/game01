pub mod resources {
    use bevy::{ecs::system::Resource, time::Timer};

    //用户移动定时器
    #[derive(Debug, Resource)]
    pub struct PlayerMoveTimer(pub Timer);
}
pub mod components {
    use bevy::ecs::component::Component;

    #[derive(Debug, Component)]
    pub struct Player;
}
pub mod systems {
    use bevy::{
        asset::{AssetServer, Assets},
        ecs::system::{Commands, Res, ResMut},
        math::{Vec3, primitives::Circle},
        render::mesh::{Mesh, Mesh2d},
        sprite::{ColorMaterial, MeshMaterial2d},
        transform::components::Transform,
        utils::default,
    };

    use crate::components::Hp;

    use super::components::Player;

    pub fn add_player(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        assets_server: Res<AssetServer>,
    ) {
        commands.spawn((
            Player,
            Hp(100.0),
            Mesh2d(meshes.add(Circle::new(25.0))),
            MeshMaterial2d(
                materials.add(ColorMaterial::from(assets_server.load("textures/wj.jpg"))),
            ),
            Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
        ));
    }
}
pub mod plugins {
    use super::{resources::PlayerMoveTimer, systems::add_player};
    use bevy::{
        app::{Plugin, Startup},
        time::{Timer, TimerMode},
    };
    use std::time::Duration;

    pub struct PlayerPlugin;
    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut bevy::app::App) {
            app.insert_resource(PlayerMoveTimer(Timer::new(
                Duration::from_millis(3000),
                TimerMode::Repeating,
            )))
            .add_systems(Startup, add_player);
        }
    }
}
