<script lang="ts">
  import { get } from "svelte/store";
  import { todolist } from "../store";
  import TaskItem from "./TaskItem.svelte";

  let focused_id = 0;

  function on_key_down(e: KeyboardEvent) {
    if (e.key == "ArrowDown") {
      focused_id++;
    } else if (e.key == "ArrowUp") {
      focused_id--;
    }
  }
</script>

<div id="dropdown">
  <ul>
    {#each get(todolist) as { id, title, desc }, i}
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
