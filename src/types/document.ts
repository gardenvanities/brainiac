export interface Document {
  id: string;
  path: string;
  title: string;
  frontmatter: Record<string, unknown> | null;
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
}
