import { invoke } from "@tauri-apps/api/core";
import type {
  CreateDocumentPayload,
  Document,
  DocumentWithContent,
  RenameDocumentPayload,
  SaveDocumentPayload,
} from "../../types";

export async function createDocument(payload: CreateDocumentPayload): Promise<Document> {
  return invoke("create_document", { payload });
}

export async function renameDocument(payload: RenameDocumentPayload): Promise<Document> {
  return invoke("rename_document", { payload });
}

export async function getDocuments(): Promise<Document[]> {
  return invoke("get_documents");
}

export async function getDocument(id: string): Promise<DocumentWithContent> {
  return invoke("get_document", { id });
}

export async function saveDocument(payload: SaveDocumentPayload): Promise<Document> {
  return invoke("save_document", { payload });
}

export async function deleteDocument(id: string): Promise<boolean> {
  return invoke("delete_document", { id });
}
