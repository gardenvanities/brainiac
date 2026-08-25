export interface Document {
  id: string;
  path: string;
  title: string;
  frontmatter: Record<string, unknown> | null;
  wordCount: number;
  isDeleted: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface DocumentWithContent extends Document {
  content: string;
}
