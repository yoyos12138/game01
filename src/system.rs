use bevy::{app::{App, Startup}, ecs::{query::With, system::{Commands, Query}, world::Command}};

use crate::entity::{Name, Person};


pub fn first_system(){
    println!("ciallo! ")
}

pub fn add_persons(mut commands:Commands){
    commands.spawn((Person,Name("张三".to_string())));
    commands.spawn((Person,Name("李四".to_string())));
    commands.spawn((Person,Name("王五".to_string())));
    commands.spawn((Person,Name("赵六".to_string())));
    commands.spawn((Person,Name("田七".to_string())));
    commands.spawn((Person,Name("周八".to_string())));
    commands.spawn((Person,Name("田所浩二".to_string())));
    commands.spawn((Person,Name("周撅伦".to_string())));
    commands.spawn((Person,Name("王爷".to_string())));
}

pub fn greet_peoples(query:Query<&Name,With<Person>>){
    for name in &query{
        println!("你好，{}",name.0);
    }

}

pub fn update_people(mut query:Query<&mut Name,With<Person>>){
    for mut name in &mut query {
        if name.0=="王爷" {
            name.0="小人".to_string()
        }
    }
}