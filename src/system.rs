use bevy::{
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    time::Time,
};

use crate::{
    entity::{Name, Person},
    resource::GameTimer,
};

pub fn first_system() {
    println!("ciallo! ")
}

pub fn add_persons(mut commands: Commands) {
    commands.spawn((Person, Name("张三".to_string())));
    commands.spawn((Person, Name("李四".to_string())));
    commands.spawn((Person, Name("王五".to_string())));
    commands.spawn((Person, Name("赵六".to_string())));
    commands.spawn((Person, Name("田七".to_string())));
    commands.spawn((Person, Name("周八".to_string())));
    commands.spawn((Person, Name("田所浩二".to_string())));
    commands.spawn((Person, Name("周撅伦".to_string())));
    commands.spawn((Person, Name("王爷".to_string())));
}

pub fn greet_peoples(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("你好，{}", name.0);
    }
}

pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "王爷" {
            name.0 = "小人".to_string()
        }
    }
}

pub fn greet_with_time(
    query: Query<&Name, With<Person>>,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.0.tick(time.delta()).finished() {
        for name in &query {
            println!("啊啊啊啊啊啊啊啊我是{}", name.0)
        }
    }
}
