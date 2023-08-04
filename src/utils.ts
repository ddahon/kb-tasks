import { invoke } from "@tauri-apps/api/tauri";

function addOutsideClickHandler() {
  document.body.addEventListener("click", (event: Event) => {
    let el = event.target as HTMLElement;
    if (el && el.tagName == "BODY") {
      invoke("toggle_window_js").catch((e) => console.error(e));
    }
  });
}

function addEscHandler() {
  document.onkeydown = (e: KeyboardEvent) => {
    if (e.key == "Escape") {
      invoke("toggle_window_js").catch((e) => console.error(e));
    }
  };
}

export { addOutsideClickHandler, addEscHandler };
