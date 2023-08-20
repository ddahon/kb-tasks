use std::{
    fs::{self, File},
    io::{Error, Write},
    sync::Mutex,
};

use tauri::State;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum TaskStatus {
    Completed,
    Deleted,
    Created,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub desc: String,
    pub status: TaskStatus,
}

pub struct TodolistState(pub Mutex<Vec<Task>>);

#[tauri::command]
pub fn get_todolist(todolist: State<TodolistState>) -> Vec<Task> {
    return todolist.0.lock().unwrap().clone();
}

#[tauri::command]
pub fn add_todo(desc: &str, todolist: State<TodolistState>) {
    let mut l = todolist.0.lock().unwrap();
    let new_id = l.iter().fold(0, |acc, t| std::cmp::max(acc, t.id)) + 1;
    println!("new id: {}", new_id);
    let new_task = Task {
        id: new_id,
        title: "".to_string(),
        desc: desc.to_string(),
        status: TaskStatus::Created,
    };
    l.push(new_task.clone());
}

#[tauri::command]
pub fn complete_task(id: u32, todolist: State<TodolistState>) {
    let l: &mut Vec<Task> = &mut *todolist.0.lock().unwrap();
    for t in l {
        if t.id == id {
            (*t).status = TaskStatus::Completed;
        }
    }
}

pub fn load() -> Vec<Task> {
    let path = "/tmp/todoapp.save";
    let contents = fs::read_to_string(path);
    match contents {
        Err(_) => Vec::new(),
        Ok(s) => serde_json::from_str::<Vec<Task>>(&s).unwrap(),
    }
}

#[tauri::command]
pub fn save(todolist: State<TodolistState>) {
    let path = "/tmp/todoapp.save";

    let mut output = File::create(path).unwrap();
    write!(
        output,
        "{}",
        serde_json::to_string(&todolist.0.lock().unwrap().clone()).unwrap()
    )
    .unwrap();
}
