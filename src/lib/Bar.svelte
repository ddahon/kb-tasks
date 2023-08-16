<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { todolist } from "../store";
  import Dropdown from "./Dropdown.svelte";

  let widthVw = 50;
  let heightVh = 4;
  let input = "";
  let show_dropdown = false;
  let dropdown: HTMLElement;

  function on_focus(e: FocusEvent) {
    document.getElementById("main-bar").focus();
  }

  function on_key_down(e: KeyboardEvent) {
    if (!show_dropdown && e.key == "ArrowDown") {
      show_dropdown = true;
      dropdown.focus();
    } else if (
      show_dropdown &&
      document.activeElement == dropdown &&
      e.key == "ArrowUp"
    ) {
      show_dropdown = false;
    }
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    invoke("add_todo", {
      desc: input,
    }).then((res) => todolist.set([res as Task, ...$todolist]));
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
    />
  </form>

  {#if show_dropdown}
    <Dropdown bind:this={dropdown} />
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
