import { createDocument, getDocument, getDocuments, saveDocument } from "../lib/tauri/documents";
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
}

export const documentsStore = new DocumentsStore();
