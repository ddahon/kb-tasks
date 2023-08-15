<script lang="ts">
  import Bar from "./lib/Bar.svelte";
  import { addOutsideClickHandler, addEscHandler } from "./utils";
  import { invoke } from "@tauri-apps/api/tauri";
  import { todolist } from "./store";

  addOutsideClickHandler();
  addEscHandler();

  invoke("get_todolist").then((res) => todolist.set(res as Task[]));
</script>

<main>
  <Bar />
</main>
