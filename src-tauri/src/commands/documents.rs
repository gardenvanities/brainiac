use crate::database::queries;
use crate::database::DbState;
use crate::error::AppError;
use crate::filesystem;
use crate::filesystem::frontmatter::{compose_document, split_frontmatter, set_title_in_fm};
use crate::models::document::{
    CreateDocumentPayload, Document, DocumentWithContent, RenameDocumentPayload,
    SaveDocumentPayload,
};
use chrono::Utc;
use libsql::Connection;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

fn resolve_files_dir(app: &AppHandle) -> Result<PathBuf, AppError> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Internal(e.to_string()))?
        .join("files");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

// ---------- lógica pura / testável ----------

/// Normaliza o nome informado no título inline: strip de `.md`,
/// espaço → `_`, rejeita vazio/`.`, `..` e separadores de caminho.
pub fn normalize_document_name(raw: &str) -> Result<String, AppError> {
    let mut name = raw.trim().to_string();
    if let Some(stem) = name.strip_suffix(".md") {
        name = stem.trim().to_string();
    }
    let name = name.replace(' ', "_");
    if name.is_empty() || name == "." || name == ".." || name.contains('/') || name.contains('\\')
    {
        return Err(AppError::Validation(
            "Nome de arquivo inválido".to_string(),
        ));
    }
    Ok(name)
}

/// Atualiza (ou insere) `title` no frontmatter JSON do banco,
/// preservando os demais campos. JSON inválido é substituído.
fn merge_title_into_fm_json(fm: Option<&str>, title: &str) -> Option<String> {
    let mut obj: serde_json::Map<String, serde_json::Value> = fm
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
    obj.insert(
        "title".to_string(),
        serde_json::Value::String(title.to_string()),
    );
    serde_json::to_string(&obj).ok()
}

/// Lê o arquivo e retorna apenas o corpo, sem o bloco de frontmatter —
/// é o conteúdo que o editor enxerga.
fn read_body(path: &str) -> Result<String, AppError> {
    let full = filesystem::documents::read_file(path)?;
    Ok(split_frontmatter(&full).1)
}

/// Cenário A — renomeia o arquivo no disco e a row no banco.
/// Conflito quando o caminho alvo existe em disco OU pertence a outra row.
/// Nome equivalente ao atual é no-op (idempotente).
pub async fn rename_document_impl(
    conn: &Connection,
    files_dir: &Path,
    id: &str,
    new_name: &str,
) -> Result<Document, AppError> {
    let name = normalize_document_name(new_name)?;

    let doc = queries::documents::get_by_id(conn, id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Documento {} não encontrado", id)))?;

    let current_stem = Path::new(&doc.path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string();
    if name == current_stem {
        return Ok(doc);
    }

    let target = files_dir.join(format!("{name}.md"));
    let target_str = target.to_string_lossy().to_string();
    if target.exists() || queries::documents::path_exists(conn, &target_str).await? {
        return Err(AppError::Conflict(
            "Já existe um arquivo com este nome".to_string(),
        ));
    }

    filesystem::documents::rename_file(&doc.path, &target_str)?;

    let now = Utc::now().to_rfc3339();
    queries::documents::update_path_and_title(conn, id, &target_str, &name, &now).await?;

    queries::documents::get_by_id(conn, id)
        .await?
        .ok_or_else(|| AppError::NotFound("Documento não encontrado após rename".to_string()))
}

/// Persiste corpo + frontmatter. O backend é o único escritor do arquivo:
/// o editor entrega só o corpo; o frontmatter existente (do disco) é
/// preservado, e `payload.title` (Cenário B) atualiza a linha `title:`.
pub async fn save_document_impl(
    conn: &Connection,
    payload: &SaveDocumentPayload,
) -> Result<Document, AppError> {
    let doc = queries::documents::get_by_id(conn, &payload.id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Documento {} não encontrado", payload.id)))?;

    let (existing_fm, _body) = split_frontmatter(&filesystem::documents::read_file(&doc.path)?);
    let fm_text = payload
        .title
        .as_deref()
        .map(|t| set_title_in_fm(existing_fm.as_deref(), t))
        .or(existing_fm);

    filesystem::documents::write_file(
        &doc.path,
        &compose_document(fm_text.as_deref(), &payload.content),
    )?;

    let frontmatter_json = match &payload.title {
        Some(t) => merge_title_into_fm_json(doc.frontmatter.as_deref(), t),
        None => doc.frontmatter.clone(),
    };
    let title = payload.title.clone().unwrap_or_else(|| doc.title.clone());
    let word_count = filesystem::documents::count_words(&payload.content);
    let now = Utc::now().to_rfc3339();

    queries::documents::update_meta(
        conn,
        &payload.id,
        &title,
        frontmatter_json.as_deref(),
        word_count,
        &now,
    )
    .await?;

    queries::documents::get_by_id(conn, &payload.id)
        .await?
        .ok_or_else(|| AppError::NotFound("Documento não encontrado após save".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::migrations;
    use libsql::Connection;
    use std::path::PathBuf;

    // ---------- helpers ----------

    fn temp_files_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("brainiac-test-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    async fn test_conn() -> Connection {
        let db = libsql::Builder::new_local(":memory:").build().await.unwrap();
        let conn = db.connect().unwrap();
        migrations::run(&conn).await.unwrap();
        conn
    }

    async fn insert_disk_doc(
        conn: &Connection,
        dir: &PathBuf,
        id: &str,
        name: &str,
        content: &str,
    ) -> Document {
        let path = dir.join(format!("{name}.md"));
        std::fs::write(&path, content).unwrap();
        let doc = Document {
            id: id.to_string(),
            path: path.to_string_lossy().to_string(),
            title: name.to_string(),
            frontmatter: None,
            word_count: 0,
            is_deleted: false,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        };
        queries::documents::insert(conn, &doc).await.unwrap();
        doc
    }

    // ---------- normalize_document_name ----------

    #[test]
    fn normaliza_espacos_para_underscore() {
        assert_eq!(normalize_document_name("Minha Nota").unwrap(), "Minha_Nota");
    }

    #[test]
    fn remove_sufixo_md() {
        assert_eq!(normalize_document_name("nota.md").unwrap(), "nota");
    }

    #[test]
    fn rejeita_vazio_e_somente_extensao() {
        assert!(normalize_document_name("").is_err());
        assert!(normalize_document_name("   .md ").is_err());
        assert!(normalize_document_name(".").is_err());
        assert!(normalize_document_name("..").is_err());
    }

    #[test]
    fn rejeita_barra_no_nome() {
        assert!(normalize_document_name("a/b").is_err());
        assert!(normalize_document_name("a\\b").is_err());
    }

    // ---------- rename_document_impl (Cenário A) ----------

    #[tokio::test]
    async fn rename_move_arquivo_e_atualiza_row() {
        let conn = test_conn().await;
        let dir = temp_files_dir();
        insert_disk_doc(&conn, &dir, "id-1", "antigo", "corpo").await;

        let doc = rename_document_impl(&conn, &dir, "id-1", "Novo Nome").await.unwrap();

        assert!(dir.join("Novo_Nome.md").exists());
        assert!(!dir.join("antigo.md").exists());
        assert_eq!(doc.path, dir.join("Novo_Nome.md").to_string_lossy());
        assert_eq!(doc.title, "Novo_Nome");
    }

    #[tokio::test]
    async fn rename_conflito_em_disco_mantem_estado_consistente() {
        let conn = test_conn().await;
        let dir = temp_files_dir();
        insert_disk_doc(&conn, &dir, "id-1", "antigo", "A").await;
        insert_disk_doc(&conn, &dir, "id-outro", "ocupado", "B").await;

        let err = rename_document_impl(&conn, &dir, "id-1", "ocupado.md").await.unwrap_err();

        assert!(matches!(err, AppError::Conflict(m) if m.contains("Já existe um arquivo com este nome")));
        // estado consistente: nada mudou
        assert!(dir.join("antigo.md").exists());
        let doc = queries::documents::get_by_id(&conn, "id-1").await.unwrap().unwrap();
        assert_eq!(doc.path, dir.join("antigo.md").to_string_lossy());
    }

    #[tokio::test]
    async fn rename_conflito_no_banco_para_arquivo_orfao() {
        let conn = test_conn().await;
        let dir = temp_files_dir();
        insert_disk_doc(&conn, &dir, "id-1", "antigo", "A").await;
        // "fantasma.md" existe no BANCO (mesmo files_dir) mas o arquivo
        // não está mais no disco — row órfã também é conflito
        let doc_fantasma = insert_disk_doc(&conn, &dir, "id-2", "fantasma", "F").await;
        std::fs::remove_file(&doc_fantasma.path).unwrap();

        let err = rename_document_impl(&conn, &dir, "id-1", "fantasma").await.unwrap_err();

        assert!(matches!(err, AppError::Conflict(m) if m.contains("Já existe um arquivo com este nome")));
        assert!(dir.join("antigo.md").exists());
    }

    #[tokio::test]
    async fn rename_no_op_quando_nome_igual() {
        let conn = test_conn().await;
        let dir = temp_files_dir();
        insert_disk_doc(&conn, &dir, "id-1", "antigo", "A").await;

        let doc = rename_document_impl(&conn, &dir, "id-1", "antigo.md").await.unwrap();

        assert_eq!(doc.path, dir.join("antigo.md").to_string_lossy());
        assert!(dir.join("antigo.md").exists());
    }

    #[tokio::test]
    async fn rename_404_para_id_inexistente() {
        let conn = test_conn().await;
        let dir = temp_files_dir();
        let err = rename_document_impl(&conn, &dir, "nada", "qualquer").await.unwrap_err();
        assert!(matches!(err, AppError::NotFound(_)));
    }

    // ---------- save_document_impl / pipeline frontmatter (Cenário B) ----------

    #[tokio::test]
    async fn save_com_title_compoe_frontmatter_e_body_sem_fm() {
        let conn = test_conn().await;
        let dir = temp_files_dir();
        insert_disk_doc(&conn, &dir, "id-1", "antigo", "# nota").await;

        let payload = SaveDocumentPayload {
            id: "id-1".to_string(),
            content: "# nota v2".to_string(),
            title: Some("Minha".to_string()),
        };
        let doc = save_document_impl(&conn, &payload).await.unwrap();

        let on_disk = std::fs::read_to_string(&doc.path).unwrap();
        assert_eq!(on_disk, "---\ntitle: Minha\n---\n# nota v2");
        assert_eq!(doc.frontmatter.as_deref(), Some("{\"title\":\"Minha\"}"));
        // o editor recebe só o corpo
        assert_eq!(read_body(&doc.path).unwrap(), "# nota v2");
    }

    #[tokio::test]
    async fn save_sem_title_preserva_frontmatter_existente() {
        let conn = test_conn().await;
        let dir = temp_files_dir();
        insert_disk_doc(&conn, &dir, "id-1", "antigo", "---\ntitle: Antiga\ntags: a\n---\n# corpo").await;
        queries::documents::update_meta(
            &conn, "id-1", "antigo", Some("{\"title\":\"Antiga\",\"tags\":\"a\"}"), 2, "2026-01-01T00:00:00Z",
        )
        .await
        .unwrap();

        let payload = SaveDocumentPayload {
            id: "id-1".to_string(),
            content: "# corpo v2".to_string(),
            title: None, // autosave não mexe no título
        };
        let doc = save_document_impl(&conn, &payload).await.unwrap();

        let on_disk = std::fs::read_to_string(&doc.path).unwrap();
        assert!(on_disk.starts_with("---\ntitle: Antiga\ntags: a\n---\n"));
        assert!(on_disk.ends_with("# corpo v2"));
        assert_eq!(doc.frontmatter.as_deref(), Some("{\"title\":\"Antiga\",\"tags\":\"a\"}"));
    }

    #[test]
    fn merge_title_into_fm_json_cobre_os_casos() {
        assert_eq!(merge_title_into_fm_json(None, "X").as_deref(), Some("{\"title\":\"X\"}"));
        assert_eq!(merge_title_into_fm_json(Some("{}"), "X").as_deref(), Some("{\"title\":\"X\"}"));
        assert_eq!(
            merge_title_into_fm_json(Some("{\"title\":\"Antiga\"}"), "Nova").as_deref(),
            Some("{\"title\":\"Nova\"}")
        );
        assert_eq!(merge_title_into_fm_json(Some("não-json"), "X").as_deref(), Some("{\"title\":\"X\"}"));
        let preserva = merge_title_into_fm_json(Some("{\"outro\":1}"), "X").unwrap();
        assert!(preserva.contains("\"outro\":1") && preserva.contains("\"title\":\"X\""));
    }
}

#[tauri::command]
pub async fn create_document(
    app: AppHandle,
    state: State<'_, DbState>,
    payload: CreateDocumentPayload,
) -> Result<Document, AppError> {
    let files_dir = resolve_files_dir(&app)?;
    let file_path = files_dir.join(format!("{}.md", payload.title.replace(' ', "_")));
    let path_str = file_path.to_string_lossy().to_string();

    filesystem::documents::write_file(&path_str, "")?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let frontmatter = payload
        .frontmatter
        .map(|f| serde_json::to_string(&f))
        .transpose()?;

    let doc = Document {
        id,
        path: path_str,
        title: payload.title,
        frontmatter,
        word_count: 0,
        is_deleted: false,
        created_at: now.clone(),
        updated_at: now,
    };

    let conn = state.conn.lock().await;
    queries::documents::insert(&conn, &doc).await?;

    Ok(doc)
}

#[tauri::command]
pub async fn get_documents(state: State<'_, DbState>) -> Result<Vec<Document>, AppError> {
    let conn = state.conn.lock().await;
    queries::documents::get_all(&conn).await
}

#[tauri::command]
pub async fn get_document(
    state: State<'_, DbState>,
    id: String,
) -> Result<DocumentWithContent, AppError> {
    let conn = state.conn.lock().await;

    let doc = queries::documents::get_by_id(&conn, &id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Documento {} não encontrado", id)))?;

    let content = read_body(&doc.path)?;

    Ok(DocumentWithContent {
        document: doc,
        content,
    })
}

#[tauri::command]
pub async fn save_document(
    state: State<'_, DbState>,
    payload: SaveDocumentPayload,
) -> Result<Document, AppError> {
    let conn = state.conn.lock().await;
    save_document_impl(&conn, &payload).await
}

#[tauri::command]
pub async fn rename_document(
    app: AppHandle,
    state: State<'_, DbState>,
    payload: RenameDocumentPayload,
) -> Result<Document, AppError> {
    let files_dir = resolve_files_dir(&app)?;
    let conn = state.conn.lock().await;
    rename_document_impl(&conn, &files_dir, &payload.id, &payload.new_name).await
}

#[tauri::command]
pub async fn delete_document(state: State<'_, DbState>, id: String) -> Result<bool, AppError> {
    let conn = state.conn.lock().await;
    let now = Utc::now().to_rfc3339();
    queries::documents::soft_delete(&conn, &id, &now).await?;
    Ok(true)
}

