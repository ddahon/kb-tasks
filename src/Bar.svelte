<style>
  #main-bar {
    position: absolute;
    left: 50%;
    top: 50%;
    width: var(--width);
    height: var(--height);
    margin-top: var(--margin-top);
    margin-left: var(--margin-left);
    box-shadow: 12px 12px 5px rgba(0, 0, 0, .3);
    outline: none
  }
</style>

<script>
  let widthVw = 50;
  let heightVh = 4;
</script>

<!-- svelte-ignore a11y-autofocus -->
<input autofocus style="--width: {widthVw}vw; --height: {heightVh}vh; --margin-top: -{heightVh/2}vh; --margin-left: -{widthVw/2}vw" id="main-bar" type="text" />