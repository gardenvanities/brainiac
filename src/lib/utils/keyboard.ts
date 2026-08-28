// Matcher puro de atalhos de teclado, cross-platform.
// Gramática do combo: teclas separadas por "+", tecla principal por último.
//   "mod+b"        → Ctrl (Linux/Windows) ou Cmd (macOS) + b
//   "mod+shift+f"  → Ctrl/Cmd + Shift + f
// A combinação é EXATA: modificadores não especificados invalidam
// (evita mod+shift+f disparar também mod+f).

type ModifierKeyset = Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "shiftKey" | "altKey">;

export function isShortcut(e: ModifierKeyset, combo: string): boolean {
  const parts = combo.toLowerCase().split("+");
  const key = parts[parts.length - 1];
  const wanted = new Set(parts.slice(0, -1));

  if (e.key.toLowerCase() !== key) return false;
  const hasMod = e.ctrlKey || e.metaKey;
  if (wanted.has("mod") !== hasMod) return false;
  if (wanted.has("shift") !== e.shiftKey) return false;
  if (wanted.has("alt") !== e.altKey) return false;
  return true;
}
