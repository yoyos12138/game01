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
        asset::{AssetServer, Assets}, ecs::{query::With, system::{Commands, Query, Res, ResMut}}, input::{keyboard::KeyCode, ButtonInput}, math::{primitives::Circle, Vec3}, render::mesh::{Mesh, Mesh2d}, sprite::{ColorMaterial, MeshMaterial2d}, time::Time, transform::components::Transform, utils::default, window::{PrimaryWindow, Window}
    };

    use crate::components::{Hp, Speed};

    use super::components::Player;

    //添加玩家
    pub fn add_player(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        assets_server: Res<AssetServer>,
        main_window: Query<&Window, With<PrimaryWindow>>
    ) {
        if let Ok(m_win) = main_window.get_single() {
            let m_h=m_win.height();

            commands.spawn((
                Player,
                Hp(100.0),
                Speed(2.5),
                Mesh2d(meshes.add(Circle::new(25.0))),
                MeshMaterial2d(
                    materials.add(ColorMaterial::from(assets_server.load("textures/wj.jpg"))),
                ),
                Transform {
                    translation: Vec3::new(0.0, 25.0-m_h/2.0 , 0.0),
                    ..default()
                },
            ));
        }
    }

    //设置玩家移动
    pub fn set_player_move(
        mut player_move_timer: ResMut<super::resources::PlayerMoveTimer>,
        timer: Res<Time>,
        keys: Res<ButtonInput<KeyCode>>,
        mut player_transform:Query<(&mut Transform,&mut Speed),With< Player>>
    ) {
        if player_move_timer.0.tick(timer.delta()).finished() {
            if let Ok((mut transform,speed)) = player_transform.get_single_mut() {
                let ratio0=2.0_f32.sqrt();
                match (
                    keys.pressed(KeyCode::KeyW), 
                    keys.pressed(KeyCode::KeyA), 
                    keys.pressed(KeyCode::KeyS), 
                    keys.pressed(KeyCode::KeyD)){
                        //上
                        (true,a,false,d)if a==d=>{
                            transform.translation.y += speed.0;
                        }
                        //下
                        (false,a,true,d)if a==d=>{
                            transform.translation.y -= speed.0;
                        }
                        //左
                        (w,true,s,false)if w==s=>{
                            transform.translation.x -= speed.0;
                        }
                        //右
                        (w,false,s,true)if w==s=>{
                            transform.translation.x += speed.0;
                        }
                        //上左
                        (true,true,false,false)=>{
                            transform.translation.x -= speed.0/ratio0;
                            transform.translation.y += speed.0/ratio0;
                        }
                        //上右
                        (true,false,false,true)=>{
                            transform.translation.x += speed.0/ratio0;
                            transform.translation.y += speed.0/ratio0;
                        }
                        //下左
                        (false,true,true,false)=>{
                            transform.translation.x -= speed.0/ratio0;
                            transform.translation.y -= speed.0/ratio0;
                        }
                        //下右
                        (false,false,true,true)=>{
                            transform.translation.x += speed.0/ratio0;
                            transform.translation.y -= speed.0/ratio0;
                        }
                        _=>{}
                    }
            }
        }
    }


    //限制玩家移动在窗户内
    pub fn limit_player_move(
        mut player_transform:Query<&mut Transform,With< Player>>,
        main_window: Query<&Window, With<PrimaryWindow>>
    ) {
        if let Ok(m_win) = main_window.get_single() {
            if let Ok(mut m_p) = player_transform.get_single_mut() {
                m_p.translation.x = m_p.translation.x.clamp(f32::MIN, m_win.width() / 2.0 - 25.0);
                m_p.translation.y = m_p.translation.y.clamp(f32::MIN, m_win.height() / 2.0 - 25.0);
                m_p.translation.x = m_p.translation.x.clamp(25.0-m_win.width() / 2.0 , f32::MAX);
                
                if m_p.translation.y <25.0-m_win.height() / 2.0 {
                    m_p.translation.y =25.0-m_win.height()/2.0;
                    
                }
                
            }
        }
    }

}
pub mod plugins {
    use super::{resources::PlayerMoveTimer, systems::{add_player, limit_player_move, set_player_move}};
    use bevy::{
        app::{Plugin, Startup, Update},
        time::{Timer, TimerMode},
    };
    use std::time::Duration;

    pub struct PlayerPlugin;
    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut bevy::app::App) {
            app
            .insert_resource(PlayerMoveTimer(Timer::new(Duration::from_micros(100),TimerMode::Repeating)))
            .add_systems(Startup, add_player)
            .add_systems(Update, set_player_move)
            .add_systems(Update, limit_player_move);

        }
    }
}
