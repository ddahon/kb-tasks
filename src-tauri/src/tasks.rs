use std::sync::Mutex;

use tauri::State;

#[derive(Clone)]
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
