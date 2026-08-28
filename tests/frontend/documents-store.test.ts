import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  attachEditorSurrogate,
  flushReactivity,
} from "./helpers/editor-surrogate.svelte";
import { documentsStore } from "../../src/stores/documents.store.svelte";

// ---------------------------------------------------------------------------
// Backend fake — espelha o CONTRATO dos commands Rust (não a implementação):
// - Document retornado NUNCA carrega `content` (só DocumentWithContent tem);
// - rename move o arquivo no "disco" preservando o conteúdo;
// - save grava o corpo no caminho atual da row.
// ---------------------------------------------------------------------------

type Row = {
  id: string;
  path: string;
  title: string;
  frontmatter: string | null;
  word_count: number;
  is_deleted: boolean;
  created_at: string;
  updated_at: string;
};

const backend = vi.hoisted(() => {
  return {
    files: new Map<string, string>(),
    rows: new Map<string, Row>(),
    saveCalls: [] as { id: string; content: string; title?: string }[],
    renameCalls: [] as { id: string; newName: string }[],
    failNextRename: false,
    reset() {
      this.files.clear();
      this.rows.clear();
      this.saveCalls.length = 0;
      this.renameCalls.length = 0;
      this.failNextRename = false;
    },
  };
});

function nowStamp(): string {
  return "2026-02-02T00:00:00Z";
}

function makeRow(id: string, path: string, title: string): Row {
  return {
    id,
    path,
    title,
    frontmatter: null,
    word_count: 0,
    is_deleted: false,
    created_at: "2026-01-01T00:00:00Z",
    updated_at: nowStamp(),
  };
}

vi.mock("../../src/lib/tauri/documents", () => ({
  async createDocument(payload: { title: string }) {
    const path = `/files/${payload.title}.md`;
    backend.files.set(path, "");
    const row = makeRow(`id-${backend.rows.size + 1}`, path, payload.title);
    backend.rows.set(row.id, row);
    return { ...row };
  },

  async getDocuments(): Promise<Row[]> {
    return [...backend.rows.values()].map((r) => ({ ...r }));
  },

  async getDocument(id: string) {
    const row = backend.rows.get(id);
    if (!row) throw new Error(`Documento ${id} não encontrado`);
    return { ...row, content: backend.files.get(row.path) ?? "" };
  },

  async saveDocument(payload: { id: string; content: string; title?: string }) {
    backend.saveCalls.push({ ...payload });
    const row = backend.rows.get(payload.id);
    if (!row) throw new Error(`Documento ${payload.id} não encontrado`);
    backend.files.set(row.path, payload.content);
    if (payload.title !== undefined) {
      row.title = payload.title;
      row.frontmatter = JSON.stringify({ title: payload.title });
    }
    row.updated_at = nowStamp();
    return { ...row };
  },

  async renameDocument(payload: { id: string; newName: string }) {
    backend.renameCalls.push({ ...payload });
    if (backend.failNextRename) {
      backend.failNextRename = false;
      throw new Error("Já existe um arquivo com este nome");
    }
    const row = backend.rows.get(payload.id);
    if (!row) throw new Error(`Documento ${payload.id} não encontrado`);
    const stem = payload.newName.replace(/\.md$/i, "");
    const newPath = `/files/${stem}.md`;
    // rename no disco preserva o conteúdo do arquivo
    backend.files.set(newPath, backend.files.get(row.path) ?? "");
    backend.files.delete(row.path);
    row.path = newPath;
    row.title = stem;
    row.updated_at = nowStamp();
    return { ...row };
  },

  async deleteDocument(): Promise<boolean> {
    return true;
  },
}));

function resetStore() {
  documentsStore.list = [];
  documentsStore.active = null;
  documentsStore.currentContent = "";
  documentsStore.error = null;
  documentsStore.saving = false;
  documentsStore.docLoading = false;
  documentsStore.listLoading = false;
}

beforeEach(() => {
  backend.reset();
  resetStore();
});

describe("DocumentsStore.rename — rename é semanticamente distinto de save", () => {
  it("atualiza metadados sem reconstruir o documento ativo (referência preservada)", async () => {
    await documentsStore.create("Nota Teste");
    const before = documentsStore.active;

    const ok = await documentsStore.rename(before!.id, "Novo Título");

    expect(ok).toBe(true);
    expect(documentsStore.active).toBe(before); // mesmo objeto — nada reconstruído
    expect(documentsStore.active?.path).toBe("/files/Novo Título.md");
    expect(documentsStore.active?.title).toBe("Novo Título");
  });

  it("não altera o snapshot de conteúdo do documento ativo nem o conteúdo corrente", async () => {
    await documentsStore.create("Nota Teste");
    documentsStore.updateContent("Conteúdo X");
    await documentsStore.save("Conteúdo X");
    const contentSnapshotBefore = documentsStore.active!.content;
    const currentBefore = documentsStore.currentContent;

    await documentsStore.rename(documentsStore.active!.id, "Título B");

    expect(documentsStore.active?.content).toBe(contentSnapshotBefore); // intocado pelo rename
    expect(documentsStore.currentContent).toBe(currentBefore);
    expect(documentsStore.currentContent).toBe("Conteúdo X");
  });

  it("rename repetido A→B→C preserva o conteúdo em todas as etapas", async () => {
    await documentsStore.create("Titulo A");
    const id = documentsStore.active!.id;
    documentsStore.updateContent("Conteúdo X");
    await documentsStore.save("Conteúdo X");

    await documentsStore.rename(id, "Titulo B");
    expect(documentsStore.currentContent).toBe("Conteúdo X");

    await documentsStore.rename(id, "Titulo C");
    expect(documentsStore.currentContent).toBe("Conteúdo X");
    expect(documentsStore.active?.path).toBe("/files/Titulo C.md");
  });

  it("não envia content ao backend — rename não é save", async () => {
    await documentsStore.create("Nota Teste");
    documentsStore.updateContent("Conteúdo X");
    await documentsStore.save("Conteúdo X");

    await documentsStore.rename(documentsStore.active!.id, "Novo Nome");

    expect(backend.renameCalls).toHaveLength(1);
    expect(backend.renameCalls[0]).not.toHaveProperty("content");
    expect(backend.saveCalls).toHaveLength(1); // apenas o autosave do preparo
  });

  it("falha de rename mantém estado local consistente e expõe erro", async () => {
    await documentsStore.create("Nota Teste");
    documentsStore.updateContent("Conteúdo X");
    await documentsStore.save("Conteúdo X");
    const before = documentsStore.active;
    backend.failNextRename = true;

    const ok = await documentsStore.rename(before!.id, "Outro Nome");

    expect(ok).toBe(false);
    expect(documentsStore.error).toContain("Já existe um arquivo");
    expect(documentsStore.active).toBe(before);
    expect(documentsStore.currentContent).toBe("Conteúdo X");
    expect(documentsStore.active?.path).toBe("/files/Nota Teste.md");
  });

  it("sincroniza o nó da lista (sidebar) com o novo caminho", async () => {
    await documentsStore.create("Nota Teste");
    const id = documentsStore.active!.id;

    await documentsStore.rename(id, "Renomeado");

    const node = documentsStore.list.find((d) => d.id === id);
    expect(node?.path).toBe("/files/Renomeado.md");
  });
});

describe("DocumentsStore.updateTitle — Cenário B (frontmatter)", () => {
  it("persiste o conteúdo corrente junto ao título sem reconstruir o documento", async () => {
    await documentsStore.create("Nota Teste");
    documentsStore.updateContent("Conteúdo X");
    await documentsStore.save("Conteúdo X");
    // simula frontmatter title já existente (Cenário B)
    documentsStore.active!.frontmatter = JSON.stringify({ title: "Nota Teste" });
    const before = documentsStore.active;

    const ok = await documentsStore.updateTitle(before!.id, "Minha Nota");

    expect(ok).toBe(true);
    expect(documentsStore.active).toBe(before); // mesma referência — nada reconstruído
    expect(documentsStore.currentContent).toBe("Conteúdo X");
    expect(backend.saveCalls.at(-1)?.content).toBe("Conteúdo X");
    expect(backend.saveCalls.at(-1)?.title).toBe("Minha Nota");
  });
});

describe("regressão: renomear não pode apagar o conteúdo do editor", () => {
  // Reproduz o cenário do bug: nota existente com conteúdo → usuário edita
  // o título → rename/updateTitle SUBSTITUÍAM o objeto `active`; o $effect
  // do Editor rastreia o campo `active`, então o Milkdown era destruído e
  // recriado com `active.content` — o snapshot da ABERTURA, que fica
  // obsoleto após o primeiro autosave. O editor voltava a mostrar o
  // conteúdo antigo/vazio e o próximo autosave persistia a perda no disco.
  it("nota existente: abrir → conteúdo → autosave → renomear → sem rebuild; recarregar mantém conteúdo", async () => {
    // nota existente no disco com conteúdo
    await documentsStore.create("Nota Teste");
    const path = documentsStore.active!.path;
    backend.files.set(path, "Conteúdo X");

    // abrir a nota existente
    await documentsStore.open(documentsStore.active!.id);
    expect(documentsStore.active?.content).toBe("Conteúdo X");

    const surrogate = await attachEditorSurrogate(documentsStore);
    expect(surrogate.creations).toBe(1); // abertura criou o editor
    expect(surrogate.contentsAtCreation.at(-1)).toBe("Conteúdo X");

    // usuário edita o conteúdo → observer → autosave
    documentsStore.updateContent("Conteúdo X editado");
    await documentsStore.save("Conteúdo X editado");
    await flushReactivity();
    expect(surrogate.creations).toBe(1); // autosave não reconstrói

    // usuário edita SOMENTE o título
    const ok = await documentsStore.rename(documentsStore.active!.id, "Minha Nota");
    await flushReactivity();

    expect(ok).toBe(true);
    // o editor NÃO pode ser recriado por um rename…
    expect(surrogate.creations).toBe(1);
    // …e o conteúdo corrente permanece o digitado pelo usuário
    expect(documentsStore.currentContent).toBe("Conteúdo X editado");

    // recarregar a aplicação (reabrir do disco)
    await documentsStore.open(documentsStore.active!.id);
    await flushReactivity();
    expect(documentsStore.active?.content).toBe("Conteúdo X editado");
    expect(documentsStore.currentContent).toBe("Conteúdo X editado");
    expect(documentsStore.active?.title).toBe("Minha Nota");
  });

  it("nota nova: renomear após digitar não deixa o editor com conteúdo obsoleto", async () => {
    await documentsStore.create("Nota Teste");
    const id = documentsStore.active!.id;

    const surrogate = await attachEditorSurrogate(documentsStore);
    expect(surrogate.creations).toBe(1);

    documentsStore.updateContent("Conteúdo X");
    await documentsStore.save("Conteúdo X");
    await flushReactivity();

    await documentsStore.rename(id, "Minha Nota");
    await flushReactivity();

    expect(surrogate.creations).toBe(1);
    expect(surrogate.contentsAtCreation.at(-1)).toBe("");

    await documentsStore.open(id);
    await flushReactivity();
    expect(documentsStore.active?.content).toBe("Conteúdo X");
  });
});

describe("evidência do mecanismo (Svelte 5)", () => {
  // Trava a semântica em que o bug se apoiava: substituir o objeto `active`
  // (mesmo id) recria o editor — e a recriação lê `active.content`, não o
  // conteúdo corrente do editor. Se este teste falhar, a semântica do
  // $effect mudou e o surrogate precisa ser revisto.
  it("substituir o objeto active com o mesmo id recria o editor com o conteúdo do snapshot", async () => {
    await documentsStore.create("Nota Teste");
    documentsStore.updateContent("Texto digitado pelo usuário");

    const surrogate = await attachEditorSurrogate(documentsStore);
    expect(surrogate.creations).toBe(1);
    expect(surrogate.contentsAtCreation.at(-1)).toBe("");

    // padrão ANTIGO do store.rename/updateTitle: spread substituindo o objeto
    documentsStore.active = { ...documentsStore.active!, title: "Novo" };
    await flushReactivity();

    expect(surrogate.creations).toBe(2);
    expect(surrogate.contentsAtCreation.at(-1)).toBe(""); // conteúdo do snapshot, não o corrente
  });
});
