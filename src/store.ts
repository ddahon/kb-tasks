import { writable } from "svelte/store";

export const todolist = writable([]);

todolist.subscribe((value) => console.log(value));
