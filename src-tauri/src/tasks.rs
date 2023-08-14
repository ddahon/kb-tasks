use std::sync::Mutex;

use tauri::State;

#[derive(Clone, serde::Serialize)]
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
    todolist.0.lock().unwrap().push(new_task.clone());
    return new_task;
}
