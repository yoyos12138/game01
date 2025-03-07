pub mod resources {}
pub mod components {
    use bevy::ecs::component::Component;

    #[derive(Debug, Component)]
    pub struct MajorCamera;
}
pub mod systems {
    use bevy::{
        core_pipeline::core_2d::Camera2d,
        ecs::{
            query::With,
            system::{Commands, Query},
        },
        window::{PrimaryWindow, Window},
    };

    use super::components::MajorCamera;
    //关闭窗口的缩小全屏,设置窗口标题
    pub fn lock_window_size(
        mut main_window: Query<&mut Window, With<PrimaryWindow>>
    ) {
        if let Ok(mut m_win) = main_window.get_single_mut() {
            m_win.title = "test".to_string();
            m_win.resizable = false;
            m_win.enabled_buttons.maximize = false;
            //设置窗口大小
            m_win.resolution.set(600.0, 800.0);
        }
    }
    //添加摄像机
    pub fn add_camera(mut commands: Commands) {
        commands.spawn((MajorCamera, Camera2d));
    }
}
pub mod plugins {
    use super::systems::{add_camera, lock_window_size};
    use bevy::app::{Plugin, Startup};
    pub struct InitPlugin;
    impl Plugin for InitPlugin {
        fn build(&self, app: &mut bevy::app::App) {
            app.add_systems(Startup, lock_window_size);
            app.add_systems(Startup, add_camera);
        }
    }
}
