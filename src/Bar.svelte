<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let widthVw = 50;
  let heightVh = 4;

  let input = "";

  window.addEventListener("focus", () => {
    document.getElementById("main-bar").focus();
  });

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    invoke("print_rust", {
      msg: input,
    });
  }
</script>

<!-- svelte-ignore a11y-autofocus -->
<form on:submit={handleSubmit}>
  <input
    autofocus
    style="--width: {widthVw}vw; --height: {heightVh}vh; --margin-top: -{heightVh /
      2}vh; --margin-left: -{widthVw / 2}vw"
    id="main-bar"
    type="text"
    bind:value={input}
  />
</form>

<style>
  #main-bar {
    position: absolute;
    left: 50%;
    top: 50%;
    width: var(--width);
    height: var(--height);
    margin-top: var(--margin-top);
    margin-left: var(--margin-left);
    box-shadow: 12px 12px 5px rgba(0, 0, 0, 0.3);
    outline: none;
  }
</style>
