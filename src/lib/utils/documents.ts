// Helpers puros de documento — lógica de exibição do Inline Title
// e do painel lateral, e atualização cirúrgica da lista.
// Sem dependências de store/Tauri para serem testáveis via Vitest.

import type { Document, DocumentWithContent } from "../../types";

type AnyDoc = Document | DocumentWithContent;

/// Extrai `title:` do frontmatter JSON do documento.
/// Retorna null quando ausente, inválido ou vazio.
export function parseFrontmatterTitle(doc: AnyDoc): string | null {
  if (!doc.frontmatter) return null;
  try {
    const obj = JSON.parse(doc.frontmatter) as { title?: unknown };
    if (typeof obj?.title === "string" && obj.title.trim().length > 0) {
      return obj.title;
    }
    return null;
  } catch {
    return null;
  }
}

/// Nome físico do arquivo no disco, COM extensão — é o que o
/// painel lateral sempre exibe (identificador real do sistema).
export function fileDisplayName(path: string): string {
  const idx = path.lastIndexOf("/");
  return idx >= 0 ? path.slice(idx + 1) : path;
}

/// Título inline: frontmatter `title:` quando existe;
/// caso contrário, o nome do arquivo sem extensão.
export function resolveInlineTitle(doc: AnyDoc): string {
  const title = parseFrontmatterTitle(doc);
  if (title) return title;
  return fileDisplayName(doc.path).replace(/\.md$/i, "");
}

/// Atualização cirúrgica: substitui apenas o documento com o mesmo id,
/// preservando as referências dos demais nós e a ordem da lista.
/// Id inexistente entra no topo (mesmo comportamento do create).
export function upsertDocument<T extends { id: string }>(list: T[], updated: T): T[] {
  const index = list.findIndex((d) => d.id === updated.id);
  if (index === -1) return [updated, ...list];
  return list.map((d) => (d.id === updated.id ? updated : d));
}
