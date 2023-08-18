<script lang="ts">
  import { todolist } from "../store";
  import TaskItem from "./TaskItem.svelte";
  import { createEventDispatcher, onMount } from "svelte";

  const dispatch = createEventDispatcher();
  let dropdown;
  let focused_id = 0;
  let nb_tasks = $todolist.length;

  onMount(() => {
    dropdown.focus();
  });

  function on_key_down(e: KeyboardEvent) {
    if (e.key == "ArrowDown") {
      focused_id = (focused_id + 1) % nb_tasks;
    } else if (e.key == "ArrowUp") {
      if (focused_id == 0) {
        dispatch("close");
      }
      focused_id--;
    }
  }
</script>

<div id="dropdown" bind:this={dropdown} tabindex="-1">
  <ul>
    {#each $todolist as { id, title, desc }, i}
      <TaskItem {desc} id={i} {focused_id} />
    {/each}
  </ul>
</div>

<svelte:window on:keydown={on_key_down} />

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
