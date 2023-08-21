<script lang="ts">
  import {
    todolist,
    completeTask,
    getNotCompleted,
    deleteTask,
  } from "../store";
  import TaskItem from "./TaskItem.svelte";
  import { createEventDispatcher, onMount } from "svelte";

  export let tasks;
  const dispatch = createEventDispatcher();
  let dropdown;
  let focused_id = 0;
  $: nb_tasks = tasks.length;

  $: {
    if (nb_tasks == 0) {
      dispatch("close");
    }
  }
  $: focused_id = Math.max(Math.min(focused_id, nb_tasks - 1), 0);

  function on_key_down(e: KeyboardEvent) {
    switch (e.key) {
      case "ArrowDown":
        focused_id = (focused_id + 1) % nb_tasks;
        break;
      case "ArrowUp":
        if (focused_id == 0) {
          dispatch("close");
        }
        focused_id--;
        break;
      case "Enter":
        completeTask(tasks[focused_id].id);
        break;
      case "Delete":
      case "Backspace":
        console.log("delete");
        deleteTask(tasks[focused_id].id);
        break;
    }
  }

  onMount(() => {
    dropdown.focus();
  });
</script>

<div id="dropdown" bind:this={dropdown} tabindex="-1">
  <ul>
    {#each tasks as { id, title, desc }, i}
      <TaskItem {desc} id={i} {focused_id} />
    {/each}
  </ul>
</div>

<svelte:window
  on:keydown={on_key_down}
  on:visibilitychange={() => dispatch("close")}
/>

<style>
  #dropdown {
    min-width: 100%;
    background-color: white;
  }
  ul {
    margin: 0;
    padding: 0;
    list-style-type: none;
  }
</style>
