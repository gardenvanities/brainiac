<script lang="ts">
import { isShortcut } from "../../lib/utils/keyboard";
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

// Atalhos globais — sempre exigem Ctrl/Cmd (nunca capturam teclas simples)
// mod+B: sidebar esquerda · mod+J: sidebar de IA · mod+Shift+F: modo foco
$effect(() => {
  function onKeydown(e: KeyboardEvent) {
    if (!e.ctrlKey && !e.metaKey) return;
    if (isShortcut(e, "mod+b")) {
      e.preventDefault();
      uiStore.toggleLeft();
    } else if (isShortcut(e, "mod+j")) {
      e.preventDefault();
      uiStore.toggleRight();
    } else if (isShortcut(e, "mod+shift+f")) {
      e.preventDefault();
      uiStore.toggleFocus();
    }
  }
  window.addEventListener("keydown", onKeydown);
  return () => window.removeEventListener("keydown", onKeydown);
});
</script>

<svelte:window onmousemove={onMouseMove} onmouseup={stopDrag} />

<div
  class="app-shell"
  class:no-select={dragging !== null}
  style="
    --left-width: {uiStore.sidebarLeftWidth}px;
    --right-width: {uiStore.sidebarRightWidth}px;
    --left-visible: {uiStore.sidebarLeftOpen ? 1 : 0};
    --right-visible: {uiStore.sidebarRightOpen ? 1 : 0};
  "
>
  <!-- Os 5 filhos do grid são SEMPRE os mesmos: o painel fechado colapsa
       para largura 0 (calc(width * visible)) sem desmontar nada — o Editor
       e a conversa do chat preservam estado. -->
  <LeftSidebar />

  {#if uiStore.sidebarLeftOpen}
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
  {:else}
    <!-- sidebar fechada: a alça vira um rail discreto de reabertura -->
    <button
      class="resize-handle rail"
      title="Abrir painel esquerdo (Ctrl/Cmd+B)"
      aria-label="Abrir painel esquerdo"
      onclick={() => uiStore.toggleLeft()}
    ></button>
  {/if}

  <CenterPanel />

  {#if uiStore.sidebarRightOpen}
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
  {:else}
    <button
      class="resize-handle rail"
      title="Abrir sidebar de IA (Ctrl/Cmd+J)"
      aria-label="Abrir sidebar de IA"
      onclick={() => uiStore.toggleRight()}
    ></button>
  {/if}

  <RightSidebar />
</div>

<style>
  .app-shell {
    display: grid;
    grid-template-columns:
      calc(var(--left-width) * var(--left-visible))
      var(--resize-handle-width)
      1fr
      var(--resize-handle-width)
      calc(var(--right-width) * var(--right-visible));
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background: var(--color-bg-base);
    transition: grid-template-columns 0.2s ease;
  }

  .no-select {
    user-select: none;
    cursor: col-resize;
  }

  .resize-handle {
    background: var(--color-border-subtle);
    cursor: col-resize;
    transition: background 0.15s ease;
    z-index: 10;
  }

  .resize-handle:hover,
  .resize-handle.active {
    background: var(--color-accent-primary);
  }

  .resize-handle.rail {
    cursor: pointer;
    padding: 0;
    border: none;
  }
</style>
