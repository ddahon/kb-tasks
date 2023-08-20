<script lang="ts">
  import { addTodo, getNotCompleted, todolist } from "../store";
  import Dropdown from "./Dropdown.svelte";

  let widthVw = 50;
  let heightVh = 4;
  let input = "";
  let inputElement;
  let show_dropdown = false;
  let dropdown;

  $: notCompletedTasks = getNotCompleted($todolist);

  function on_focus(e: FocusEvent) {
    document.getElementById("main-bar").focus();
  }

  function open_dropdown() {
    show_dropdown = true;
  }

  function close_dropdown() {
    show_dropdown = false;
    inputElement.focus();
  }
  function on_key_down(e: KeyboardEvent) {
    if (
      !show_dropdown &&
      e.key == "ArrowDown" &&
      notCompletedTasks.length > 0
    ) {
      open_dropdown();
    } else if (
      show_dropdown &&
      document.activeElement == dropdown &&
      e.key == "ArrowUp"
    ) {
      close_dropdown();
    }
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    addTodo(input);
    input = "";
  }
</script>

<svelte:window on:keydown={on_key_down} on:focus={on_focus} />

<!-- svelte-ignore a11y-autofocus -->
<div
  id="main-bar-container"
  style="--width: {widthVw}vw; --margin-left: -{widthVw / 2}vw"
>
  <form on:submit={handleSubmit}>
    <input
      autofocus
      id="main-bar"
      style="--height: {heightVh}vh"
      type="text"
      bind:value={input}
      bind:this={inputElement}
    />
  </form>

  {#if show_dropdown}
    <Dropdown
      bind:this={dropdown}
      on:close={close_dropdown}
      tasks={notCompletedTasks}
    />
  {/if}
</div>

<style>
  #main-bar-container {
    position: absolute;
    left: 50%;
    top: 50%;
    width: var(--width);
    margin-left: var(--margin-left);
  }

  #main-bar {
    min-width: 100%;
    height: var(--height);
    box-shadow: 12px 12px 5px rgba(0, 0, 0, 0.3);
    outline: none;
  }
</style>
