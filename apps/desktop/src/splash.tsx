import { onCleanup, onMount } from "solid-js";
import { render } from "solid-js/web";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

function Splash() {
  onMount(() => {
    let unlisten: (() => void) | undefined;
    void listen("app-ready", async () => {
      await new Promise((r) => setTimeout(r, 420));
      await invoke("close_splash").catch(() => undefined);
    }).then((fn) => {
      unlisten = fn;
    });
    const timeout = setTimeout(() => {
      invoke("close_splash").catch(() => undefined);
    }, 14000);
    onCleanup(() => {
      clearTimeout(timeout);
      unlisten?.();
    });
  });

  return (
    <>
      <div class="mark-wrap">
        <img src="/pantheon-mark.svg" width="112" height="112" alt="" />
      </div>
      <div class="ring" aria-hidden="true" />
      <span class="label">Loading</span>
    </>
  );
}

const root = document.getElementById("splash-root");
if (root) {
  render(() => <Splash />, root);
}
