<script lang="ts">
import { uiStore } from "../../stores/ui.store.svelte";
import CenterPanel from "./CenterPanel.svelte";
import LeftSidebar from "./LeftSidebar.svelte";
import RightSidebar from "./RightSidebar.svelte";

let dragging: "left" | "right" | null = $state(null);

function onMouseMove(e: MouseEvent) {
  if (dragging === "left") {
    uiStore.sidebarLeftWidth = Math.max(180, Math.min(420, e.clientX));
  } else if (dragging === "right") {
    uiStore.sidebarRightWidth = Math.max(260, Math.min(520, window.innerWidth - e.clientX));
  }
}

function stopDrag() {
  dragging = null;
}
</script>

<svelte:window onmousemove={onMouseMove} onmouseup={stopDrag} />

<div
  class="app-shell"
  class:no-select={dragging !== null}
  style="
    --left-width: {uiStore.sidebarLeftWidth}px;
    --right-width: {uiStore.sidebarRightWidth}px;
  "
>
  <LeftSidebar />
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="resize-handle"
    class:active={dragging === "left"}
    role="separator"
    aria-orientation="vertical"
    aria-label="Redimensionar painel esquerdo"
    tabindex="0"
    onmousedown={() => (dragging = "left")}
  ></div>

  <CenterPanel />

  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="resize-handle"
    class:active={dragging === "right"}
    role="separator"
    aria-orientation="vertical"
    aria-label="Redimensionar painel direito"
    tabindex="0"
    onmousedown={() => (dragging = "right")}
  ></div>
  <RightSidebar />
</div>

<style>
  .app-shell {
    display: grid;
    grid-template-columns:
      var(--left-width)
      var(--resize-handle-width)
      1fr
      var(--resize-handle-width)
      var(--right-width);
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background: var(--bg-base);
  }

  .no-select {
    user-select: none;
    cursor: col-resize;
  }

  .resize-handle {
    background: var(--border);
    cursor: col-resize;
    transition: background 0.15s ease;
    z-index: 10;
  }

  .resize-handle:hover,
  .resize-handle.active {
    background: var(--accent);
  }
</style>
