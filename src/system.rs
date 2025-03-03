use bevy::{
    asset::{AssetServer, Assets},
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        event::EventReader,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput},
    math::{primitives::{Circle, Ellipse}, Quat, Vec2, Vec3},
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
    utils::default,
    window::{CursorMoved, PrimaryWindow, Window},
};

use crate::{
    entity::{Bullet, Life, MajorCamera, User},
    resource::{CursorPosition, UserMoveTimer, UserRadius, UserSpeed},
};

//初始化窗口
pub fn init_window(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut win0 = window_query.get_single_mut().unwrap();
    win0.title = "玉将大冒险".to_string();
    win0.enabled_buttons.maximize = false;
    win0.resizable = false;
}

//添加摄像机
pub fn add_camera2d(mut commands: Commands) {
    commands.spawn((MajorCamera, Camera2d::default()));
}

//添加玩家
pub fn add_user(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    user_radius: Res<UserRadius>,
) {
    let main_win = window_query.get_single().unwrap();
    println!("主窗口高{}宽{}", main_win.height(), main_win.width());
    let radius0 = user_radius.0;
    commands.spawn((
        User,
        Life(100.0),
        Mesh2d(meshes.add(Circle::new(radius0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(assets_server.load("textures/wj.jpg")))),
        Transform {
            translation: Vec3 {
                x: 0.0,
                y: radius0 - main_win.height() / 2.0,
                z: 0.0,
            },
            rotation: Quat::IDENTITY,
            ..default()
        },
    ));
}

//移动玩家
pub fn move_user(
    mut query_user: Query<&mut Transform, With<User>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut user_move_timer: ResMut<UserMoveTimer>,
    timer: Res<Time>,
    user_speed: Res<UserSpeed>,
) {
    let speed = user_speed.0;

    //↑
    if keys.pressed(KeyCode::KeyW)
        && !keys.pressed(KeyCode::KeyS)
        && !keys.pressed(KeyCode::KeyA)
        && !keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.y += speed;
        }
    }

    //↓
    if keys.pressed(KeyCode::KeyS)
        && !keys.pressed(KeyCode::KeyW)
        && !keys.pressed(KeyCode::KeyA)
        && !keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.y -= speed;
        }
    }

    //←
    if keys.pressed(KeyCode::KeyA)
        && !keys.pressed(KeyCode::KeyW)
        && !keys.pressed(KeyCode::KeyS)
        && !keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.x -= speed;
        }
    }

    //→
    if keys.pressed(KeyCode::KeyD)
        && !keys.pressed(KeyCode::KeyW)
        && !keys.pressed(KeyCode::KeyS)
        && !keys.pressed(KeyCode::KeyA)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.x += speed;
        }
    }

    let diagonal_ratio = 2.0_f32.sqrt();
    //斜率
    //↖
    if keys.pressed(KeyCode::KeyW)
        && keys.pressed(KeyCode::KeyA)
        && !keys.pressed(KeyCode::KeyS)
        && !keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.y += speed / diagonal_ratio;
            user0.translation.x -= speed / diagonal_ratio;
        }
    }

    //↗
    if keys.pressed(KeyCode::KeyW)
        && keys.pressed(KeyCode::KeyD)
        && !keys.pressed(KeyCode::KeyS)
        && !keys.pressed(KeyCode::KeyA)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.y += speed / diagonal_ratio;
            user0.translation.x += speed / diagonal_ratio;
        }
    }

    //↙
    if keys.pressed(KeyCode::KeyS)
        && keys.pressed(KeyCode::KeyA)
        && !keys.pressed(KeyCode::KeyW)
        && !keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.y -= speed / diagonal_ratio;
            user0.translation.x -= speed / diagonal_ratio;
        }
    }

    //↘
    if keys.pressed(KeyCode::KeyS)
        && keys.pressed(KeyCode::KeyD)
        && !keys.pressed(KeyCode::KeyW)
        && !keys.pressed(KeyCode::KeyA)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.y -= speed / diagonal_ratio;
            user0.translation.x += speed / diagonal_ratio;
        }
    }

    //↕←
    if keys.pressed(KeyCode::KeyW)
        && keys.pressed(KeyCode::KeyS)
        && keys.pressed(KeyCode::KeyA)
        && !keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.x -= speed;
        }
    }

    //↕→
    if keys.pressed(KeyCode::KeyW)
        && keys.pressed(KeyCode::KeyS)
        && !keys.pressed(KeyCode::KeyA)
        && keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.x += speed;
        }
    }

    //↔︎↑
    if keys.pressed(KeyCode::KeyW)
        && keys.pressed(KeyCode::KeyA)
        && !keys.pressed(KeyCode::KeyS)
        && keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.y += speed;
        }
    }

    //↔︎↓
    if keys.pressed(KeyCode::KeyS)
        && keys.pressed(KeyCode::KeyA)
        && !keys.pressed(KeyCode::KeyW)
        && keys.pressed(KeyCode::KeyD)
    {
        if user_move_timer.0.tick(timer.delta()).finished() {
            let mut user0 = query_user.single_mut();
            user0.translation.y -= speed;
        }
    }
}

//限制玩家在窗口内
pub fn limit_user_in_windows(
    mut query_user: Query<&mut Transform, With<User>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    user_radius: Res<UserRadius>,
) {
    let win0 = window_query.get_single().unwrap();
    let mut user0 = query_user.get_single_mut().unwrap();
    let radius0 = user_radius.0;

    //上
    if user0.translation.y > win0.height() / 2.0 - radius0 {
        user0.translation.y = win0.height() / 2.0 - radius0;
    }
    //下
    if user0.translation.y < 0.0 - win0.height() / 2.0 + radius0 {
        user0.translation.y = 0.0 - win0.height() / 2.0 + radius0;
    }
    //左
    if user0.translation.x < 0.0 - win0.width() / 2.0 + radius0 {
        user0.translation.x = 0.0 - win0.width() / 2.0 + radius0;
    }
    //右
    if user0.translation.x > win0.width() / 2.0 - radius0 {
        user0.translation.x = win0.width() / 2.0 - radius0;
    }
}

//设置玩家旋转
pub fn rotate_user(
    mut query_user: Query<&mut Transform, With<User>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut user_move_timer: ResMut<UserMoveTimer>,
    timer: Res<Time>,
    user_speed: Res<UserSpeed>,
) {
    //q逆时针e顺时针
    let speed0 = user_speed.0;
    let mut user0 = query_user.get_single_mut().unwrap();
    if user_move_timer.0.tick(timer.delta()).finished() {
        if keys.pressed(KeyCode::KeyJ) && !keys.pressed(KeyCode::KeyK) {
            user0.rotation *= Quat::from_rotation_z(speed0.to_radians());
        }
        if keys.pressed(KeyCode::KeyK) && !keys.pressed(KeyCode::KeyJ) {
            user0.rotation *= Quat::from_rotation_z(-speed0.to_radians());
        }
    }
}

//移动光标设置鼠标resource
pub fn set_curser_resource(
    mut cursor_moved_events: EventReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let win0 = window_query.single();
    let win_h = win0.height();
    let win_w = win0.width();
    for event in cursor_moved_events.read() {
        cursor_position.0.x = event.position.x - win_w / 2.0;
        cursor_position.0.y = win_h / 2.0 - event.position.y;
    }
}

//生成子弹
pub fn spawn_bullet(
    mut commands: Commands,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    query_user: Query<&Transform, With<User>>,
    mouse: Res<CursorPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    user_radius: Res<UserRadius>,
) {
    if mouse_btn.just_pressed(MouseButton::Left) && !mouse_btn.just_pressed(MouseButton::Right) {
        let _r = user_radius.0;
        let user0 = query_user.get_single().unwrap();
        let ux = user0.translation.x;
        let uy = user0.translation.y;
        println!("玩家位置({},{})", ux, uy);
        let cx = mouse.0.x;
        let cy = mouse.0.y;
        println!("鼠标位置({},{})", cx, cy);
        
        

        commands.spawn((
            Bullet,
            Mesh2d(meshes.add(Ellipse::new(_r/5.0, _r*2.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0)))),
            Transform::from_translation(user0.translation)
                .looking_at( Vec3::new(cx, cy, 0.0), Vec3::Y)
            //让子弹朝向鼠标的方向
        ));
    }
}
