import {
  createDocument,
  getDocument,
  getDocuments,
  renameDocument,
  saveDocument,
} from "../lib/tauri/documents";
import { upsertDocument } from "../lib/utils/documents";
import type {
  CreateDocumentPayload,
  Document,
  DocumentWithContent,
  SaveDocumentPayload,
} from "../types";

class DocumentsStore {
  list = $state<Document[]>([]);
  active = $state<DocumentWithContent | null>(null);
  currentContent = $state<string>("");
  listLoading = $state(false);
  docLoading = $state(false);
  saving = $state(false);
  error = $state<string | null>(null);

  async loadList() {
    this.listLoading = true;
    this.error = null;
    try {
      this.list = await getDocuments();
    } catch (e) {
      this.error = String(e);
    } finally {
      this.listLoading = false;
    }
  }

  updateContent(content: string) {
    this.currentContent = content;
  }

  async open(id: string) {
    // Salva conteúdo atual antes de trocar — silenciosamente
    if (this.active && this.currentContent !== this.active.content) {
      await this.save(this.currentContent);
    }

    this.docLoading = true;
    this.error = null;
    try {
      const doc = await getDocument(id);
      this.active = doc;
      this.currentContent = doc.content;
    } catch (e) {
      this.error = String(e);
    } finally {
      this.docLoading = false;
    }
  }

  async create(title: string) {
    this.error = null;
    try {
      const payload: CreateDocumentPayload = { title, path: "" };
      const doc = await createDocument(payload);
      this.list = [doc, ...this.list];
      await this.open(doc.id);
    } catch (e) {
      this.error = String(e);
    }
  }

  async save(content: string) {
    if (!this.active) return;
    this.saving = true;
    this.error = null;
    try {
      const payload: SaveDocumentPayload = { id: this.active.id, content };
      const updated = await saveDocument(payload);
      // Não atualiza this.active — evita recriar o editor
      // Só atualiza metadados na lista (word_count, updated_at)
      this.list = this.list.map((d) => (d.id === updated.id ? updated : d));
    } catch (e) {
      this.error = String(e);
    } finally {
      this.saving = false;
    }
  }

  // Atualização CIRÚRGICA dos metadados de `active`: muta apenas os campos
  // que a operação alterou — NUNCA substitui o objeto. Substituir o objeto
  // (`this.active = { ...this.active, ...updated }`) re-executa o `$effect`
  // do Editor (que rastreia o campo `active`), destruindo o Milkdown e
  // recriando-o com `active.content` — o snapshot da ABERTURA, obsoleto
  // após o primeiro autosave. Era a causa raiz do "conteúdo some ao
  // renomear". Renomear não pode reconstruir o documento.
  //
  // Deliberadamente NÃO toca em `content`: a fonte viva do conteúdo é
  // `currentContent` (atualizada pelo editor); `active.content` é o
  // snapshot carregado em `open()` e só muda quando um documento é aberto.
  #applyMetaToActive(updated: Document): void {
    if (!this.active || this.active.id !== updated.id) return;
    this.active.path = updated.path;
    this.active.title = updated.title;
    this.active.frontmatter = updated.frontmatter;
    this.active.word_count = updated.word_count;
    this.active.updated_at = updated.updated_at;
  }

  // Cenário A — renomeia o arquivo físico e o nó da sidebar.
  // Update cirúrgico: substitui apenas o nó do id na lista; `active` tem
  // APENAS os metadados mutados (o `$effect` do Editor não recria o
  // Milkdown — o documento continua aberto, intacto).
  // Retorna false em conflito/erro — `error` fica populado para a UI.
  async rename(id: string, newName: string): Promise<boolean> {
    this.error = null;
    try {
      const updated = await renameDocument({ id, newName });
      this.list = upsertDocument(this.list, updated);
      this.#applyMetaToActive(updated);
      return true;
    } catch (e) {
      this.error = String(e);
      return false;
    }
  }

  // Cenário B — atualiza o `title:` do frontmatter sem tocar na sidebar
  // (path não muda). Persiste via save_document com `title`.
  async updateTitle(id: string, title: string): Promise<boolean> {
    this.error = null;
    try {
      const payload: SaveDocumentPayload = {
        id,
        content: this.currentContent,
        title,
      };
      const updated = await saveDocument(payload);
      this.list = upsertDocument(this.list, updated);
      this.#applyMetaToActive(updated);
      return true;
    } catch (e) {
      this.error = String(e);
      return false;
    }
  }
}

export const documentsStore = new DocumentsStore();
