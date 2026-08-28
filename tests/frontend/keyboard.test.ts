import { describe, expect, it } from "vitest";
import { isShortcut } from "../../src/lib/utils/keyboard";

// Helper: KeyboardEvent não existe no Node — usar objeto mínimo tipado
function ev(partial: Partial<KeyboardEvent>): KeyboardEvent {
  return {
    key: "",
    ctrlKey: false,
    metaKey: false,
    shiftKey: false,
    altKey: false,
    ...partial,
  } as KeyboardEvent;
}

describe("isShortcut", () => {
  it("aceita mod+b com ctrlKey (Linux/Windows)", () => {
    expect(isShortcut(ev({ key: "b", ctrlKey: true }), "mod+b")).toBe(true);
  });

  it("aceita mod+b com metaKey (macOS)", () => {
    expect(isShortcut(ev({ key: "b", metaKey: true }), "mod+b")).toBe(true);
  });

  it("exige o modificador — sem ctrl/meta é falso", () => {
    expect(isShortcut(ev({ key: "b" }), "mod+b")).toBe(false);
  });

  it("mod+shift+f exige shift", () => {
    expect(isShortcut(ev({ key: "f", ctrlKey: true, shiftKey: true }), "mod+shift+f")).toBe(true);
    expect(isShortcut(ev({ key: "f", ctrlKey: true }), "mod+shift+f")).toBe(false);
  });

  it("modificador extra não especificado invalida (combinação exata)", () => {
    expect(isShortcut(ev({ key: "b", ctrlKey: true, shiftKey: true }), "mod+b")).toBe(false);
    expect(isShortcut(ev({ key: "b", ctrlKey: true, altKey: true }), "mod+b")).toBe(false);
  });

  it("tecla diferente é falso", () => {
    expect(isShortcut(ev({ key: "j", ctrlKey: true }), "mod+b")).toBe(false);
  });

  it("comparação de tecla é case-insensitive", () => {
    expect(isShortcut(ev({ key: "F", ctrlKey: true, shiftKey: true }), "mod+shift+f")).toBe(true);
  });

  it("suporta alt como modificador explícito", () => {
    expect(isShortcut(ev({ key: "x", ctrlKey: true, altKey: true }), "mod+alt+x")).toBe(true);
    expect(isShortcut(ev({ key: "x", ctrlKey: true }), "mod+alt+x")).toBe(false);
  });
});
