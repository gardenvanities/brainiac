import { describe, expect, it } from "vitest";
import { UiStore } from "../../src/stores/ui.store.svelte";

describe("UiStore — sidebars", () => {
  it("abre as duas sidebars por default", () => {
    const s = new UiStore();
    expect(s.sidebarLeftOpen).toBe(true);
    expect(s.sidebarRightOpen).toBe(true);
  });

  it("toggleLeft inverte apenas a esquerda", () => {
    const s = new UiStore();
    s.toggleLeft();
    expect(s.sidebarLeftOpen).toBe(false);
    expect(s.sidebarRightOpen).toBe(true);
  });

  it("toggleRight inverte apenas a direita", () => {
    const s = new UiStore();
    s.toggleRight();
    expect(s.sidebarLeftOpen).toBe(true);
    expect(s.sidebarRightOpen).toBe(false);
  });

  it("toggleFocus fecha ambas e restaura o layout anterior ao repetir", () => {
    const s = new UiStore();
    s.toggleLeft(); // esquerda fechada, direita aberta

    s.toggleFocus();
    expect(s.sidebarLeftOpen).toBe(false);
    expect(s.sidebarRightOpen).toBe(false);

    s.toggleFocus(); // restaura exatamente o layout de antes do foco
    expect(s.sidebarLeftOpen).toBe(false);
    expect(s.sidebarRightOpen).toBe(true);
  });

  it("toggleFocus com ambas abertas restaura ambas", () => {
    const s = new UiStore();
    s.toggleFocus();
    s.toggleFocus();
    expect(s.sidebarLeftOpen).toBe(true);
    expect(s.sidebarRightOpen).toBe(true);
  });

  it("toggleFocus com ambas fechadas individualmente restaura o último layout memorizado", () => {
    const s = new UiStore();
    s.toggleLeft();
    s.toggleRight(); // ambas fechadas sem passar pelo modo foco

    s.toggleFocus(); // restaura o snapshot (default: ambas abertas)
    expect(s.sidebarLeftOpen).toBe(true);
    expect(s.sidebarRightOpen).toBe(true);
  });
});
