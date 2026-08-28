export interface Document {
  id: string;
  path: string;
  title: string;
  /** JSON serializado do frontmatter (espelha Option<String> do Rust) */
  frontmatter: string | null;
  word_count: number;
  is_deleted: boolean;
  created_at: string;
  updated_at: string;
}

export interface DocumentWithContent extends Document {
  content: string;
}

export interface CreateDocumentPayload {
  title: string;
  path: string;
  frontmatter?: Record<string, unknown>;
}

export interface SaveDocumentPayload {
  id: string;
  content: string;
  /** Cenário B — atualiza o `title:` do frontmatter; ausente = autosave */
  title?: string;
}

export interface RenameDocumentPayload {
  id: string;
  newName: string;
}
