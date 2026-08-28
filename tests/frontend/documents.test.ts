import { describe, expect, it } from "vitest";
import {
  fileDisplayName,
  parseFrontmatterTitle,
  resolveInlineTitle,
  upsertDocument,
} from "../../src/lib/utils/documents";
import type { Document } from "../../src/types";

function doc(overrides: Partial<Document>): Document {
  return {
    id: "id-1",
    path: "/files/antigo.md",
    title: "antigo",
    frontmatter: null,
    word_count: 0,
    is_deleted: false,
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
    ...overrides,
  };
}

describe("parseFrontmatterTitle", () => {
  it("retorna title quando frontmatter tem title não vazio", () => {
    expect(parseFrontmatterTitle(doc({ frontmatter: '{"title":"Minha Nota"}' }))).toBe(
      "Minha Nota",
    );
  });

  it("retorna null sem frontmatter, com JSON inválido ou title vazio", () => {
    expect(parseFrontmatterTitle(doc({ frontmatter: null }))).toBeNull();
    expect(parseFrontmatterTitle(doc({ frontmatter: "não-json" }))).toBeNull();
    expect(parseFrontmatterTitle(doc({ frontmatter: '{"title":"   "}' }))).toBeNull();
    expect(parseFrontmatterTitle(doc({ frontmatter: '{"outro":1}' }))).toBeNull();
  });
});

describe("fileDisplayName", () => {
  it("sempre mostra o nome físico do arquivo, com extensão", () => {
    expect(fileDisplayName("/files/pasta-longa/meu-arquivo.md")).toBe("meu-arquivo.md");
    expect(fileDisplayName("meu-arquivo.md")).toBe("meu-arquivo.md");
  });
});

describe("resolveInlineTitle", () => {
  it("usa o title do frontmatter quando existe", () => {
    const d = doc({ path: "/files/antigo.md", frontmatter: '{"title":"Título Bonito"}' });
    expect(resolveInlineTitle(d)).toBe("Título Bonito");
  });

  it("caí para o nome do arquivo sem extensão quando não há title", () => {
    expect(resolveInlineTitle(doc({ path: "/files/meu-arquivo.md", frontmatter: null }))).toBe(
      "meu-arquivo",
    );
  });
});

describe("upsertDocument", () => {
  it("substitui apenas o nó com o mesmo id, preservando os demais e a ordem", () => {
    const a = doc({ id: "a", path: "/files/a.md" });
    const b = doc({ id: "b", path: "/files/b.md" });
    const b2 = doc({ id: "b", path: "/files/b-renomeado.md", title: "b-renomeado" });

    const result = upsertDocument([a, b], b2);
    expect(result).toHaveLength(2);
    expect(result[0]).toBe(a); // referência preservada
    expect(result[1].path).toBe("/files/b-renomeado.md");
  });

  it("insere no topo quando o id não existe", () => {
    const a = doc({ id: "a" });
    const novo = doc({ id: "novo" });
    const result = upsertDocument([a], novo);
    expect(result[0].id).toBe("novo");
    expect(result).toHaveLength(2);
  });
});
