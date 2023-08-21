import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";
import { TaskStatus, type Task } from "./types/task";

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

export function getNotCompleted(l: Task[]) {
  let notCompleted = l.filter((t) => t.status == TaskStatus.Created);
  console.log(notCompleted);
  return notCompleted;
}

export function deleteTask(id: number) {
  invoke("delete_task", { id }).then(refreshTodolist);
}

todolist.subscribe((value) => console.log(value));
