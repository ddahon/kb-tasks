<script>
  import { get } from "svelte/store";
  import { todolist } from "../store";
  import TaskItem from "./TaskItem.svelte";
</script>

<div id="dropdown">
  <ul>
    {#each get(todolist) as { id, title, desc }}
      <TaskItem {desc} />
    {/each}
  </ul>
</div>

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
