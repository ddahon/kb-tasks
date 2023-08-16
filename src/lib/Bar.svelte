<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { todolist } from "../store";
  import Dropdown from "./Dropdown.svelte";

  let widthVw = 50;
  let heightVh = 4;
  let input = "";
  let show_dropdown = false;

  window.addEventListener("focus", () => {
    document.getElementById("main-bar").focus();
  });

  window.addEventListener("keydown", (e) => {
    let key = (e as KeyboardEvent).key;
    if (!show_dropdown && key == "ArrowDown") {
      console.log("show dropdown");
      show_dropdown = true;
    } else if (show_dropdown && key == "ArrowUp") {
      console.log("hide dropdown");
      show_dropdown = false;
    }
  });

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    invoke("add_todo", {
      desc: input,
    }).then((res) => todolist.set([res as Task, ...$todolist]));
  }
</script>

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
    <Dropdown />
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
