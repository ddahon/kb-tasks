<script>
  export let desc = "";
  export let id = -1;
  export let focused_id = -1;

  $: background_color = focused_id == id ? "aquamarine" : "white";
</script>

<li style="background-color: {background_color}">{desc}</li>

<style>
  li {
    padding: 10px;
  }
</style>
