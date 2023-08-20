import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

export const todolist = writable([]);

export function addTodo(desc: String) {
  invoke("add_todo", { desc }).then(refreshTodolist);
}

export function completeTask(id: number) {
  invoke("complete_task", { id }).then(refreshTodolist);
}

export function refreshTodolist() {
  invoke("save");
  invoke("get_todolist").then((res) => todolist.set(res as Task[]));
}

todolist.subscribe((value) => console.log(value));
