<script>
  import Bar from "./Bar.svelte";
  import { addOutsideClickHandler, addEscHandler } from "./utils";

  addOutsideClickHandler();
  addEscHandler();
</script>

<main>
  <Bar />
</main>
