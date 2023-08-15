use std::{
    fs::{self, File},
    io::{Error, Write},
    sync::Mutex,
};

use tauri::State;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub desc: String,
}

pub struct TodolistState(pub Mutex<Vec<Task>>);

#[tauri::command]
pub fn get_todolist(todolist: State<TodolistState>) -> Vec<Task> {
    return todolist.0.lock().unwrap().clone();
}

#[tauri::command]
pub fn add_todo(desc: &str, todolist: State<TodolistState>) -> Task {
    let new_task = Task {
        id: 0,
        title: "TODO".to_string(),
        desc: desc.to_string(),
    };
    let mut l = todolist.0.lock().unwrap();
    l.push(new_task.clone());
    save(l.clone()).unwrap();
    return new_task;
}

pub fn load() -> Vec<Task> {
    let path = "/tmp/todoapp.save";
    let contents = fs::read_to_string(path);
    match contents {
        Err(_) => Vec::new(),
        Ok(s) => serde_json::from_str::<Vec<Task>>(&s).unwrap(),
    }
}

fn save(todolist: Vec<Task>) -> Result<(), Error> {
    let path = "/tmp/todoapp.save";

    let mut output = File::create(path)?;
    write!(output, "{}", serde_json::to_string(&todolist).unwrap()).unwrap();
    Ok(())
}
