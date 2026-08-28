use crate::error::AppError;
use crate::models::document::Document;
use libsql::Connection;

pub async fn insert(conn: &Connection, doc: &Document) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO documents (id, path, title, frontmatter, word_count, is_deleted, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        libsql::params![
            doc.id.clone(),
            doc.path.clone(),
            doc.title.clone(),
            doc.frontmatter.clone(),
            doc.word_count,
            doc.is_deleted as i64,
            doc.created_at.clone(),
            doc.updated_at.clone()
        ],
    )
    .await?;
    Ok(())
}

pub async fn get_all(conn: &Connection) -> Result<Vec<Document>, AppError> {
    let mut rows = conn
        .query(
            "SELECT id, path, title, frontmatter, word_count, is_deleted, created_at, updated_at
             FROM documents
             WHERE is_deleted = 0
             ORDER BY updated_at DESC",
            (),
        )
        .await?;

    let mut docs = Vec::new();
    while let Some(row) = rows.next().await? {
        docs.push(row_to_document(&row)?);
    }
    Ok(docs)
}

pub async fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Document>, AppError> {
    let mut rows = conn
        .query(
            "SELECT id, path, title, frontmatter, word_count, is_deleted, created_at, updated_at
             FROM documents
             WHERE id = ?1",
            [id],
        )
        .await?;

    if let Some(row) = rows.next().await? {
        Ok(Some(row_to_document(&row)?))
    } else {
        Ok(None)
    }
}

pub async fn update_meta(
    conn: &Connection,
    id: &str,
    title: &str,
    frontmatter: Option<&str>,
    word_count: i64,
    updated_at: &str,
) -> Result<(), AppError> {
    conn.execute(
        "UPDATE documents SET title = ?1, frontmatter = ?2, word_count = ?3, updated_at = ?4 WHERE id = ?5",
        libsql::params![
            title.to_string(),
            frontmatter.map(|s| s.to_string()),
            word_count,
            updated_at.to_string(),
            id.to_string()
        ],
    )
    .await?;
    Ok(())
}

pub async fn soft_delete(conn: &Connection, id: &str, updated_at: &str) -> Result<(), AppError> {
    conn.execute(
        "UPDATE documents SET is_deleted = 1, updated_at = ?1 WHERE id = ?2",
        [updated_at, id],
    )
    .await?;
    Ok(())
}

pub async fn update_path_and_title(
    conn: &Connection,
    id: &str,
    path: &str,
    title: &str,
    updated_at: &str,
) -> Result<(), AppError> {
    conn.execute(
        "UPDATE documents SET path = ?1, title = ?2, updated_at = ?3 WHERE id = ?4",
        libsql::params![path.to_string(), title.to_string(), updated_at.to_string(), id.to_string()],
    )
    .await?;
    Ok(())
}

pub async fn path_exists(conn: &Connection, path: &str) -> Result<bool, AppError> {
    let mut rows = conn
        .query(
            "SELECT 1 FROM documents WHERE path = ?1 AND is_deleted = 0",
            [path],
        )
        .await?;
    Ok(rows.next().await?.is_some())
}

fn row_to_document(row: &libsql::Row) -> Result<Document, AppError> {
    let is_deleted_int: i64 = row.get(5)?;
    Ok(Document {
        id: row.get(0)?,
        path: row.get(1)?,
        title: row.get(2)?,
        frontmatter: row.get(3)?,
        word_count: row.get(4)?,
        is_deleted: is_deleted_int != 0,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_conn() -> Connection {
        let db = libsql::Builder::new_local(":memory:").build().await.unwrap();
        let conn = db.connect().unwrap();
        crate::database::migrations::run(&conn).await.unwrap();
        conn
    }

    fn sample_doc(id: &str, path: &str) -> Document {
        Document {
            id: id.to_string(),
            path: path.to_string(),
            title: id.to_string(),
            frontmatter: None,
            word_count: 0,
            is_deleted: false,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    #[tokio::test]
    async fn update_path_and_title_persiste_nova_path_e_title() {
        let conn = test_conn().await;
        insert(&conn, &sample_doc("id-1", "/files/antigo.md")).await.unwrap();

        update_path_and_title(&conn, "id-1", "/files/novo.md", "novo", "2026-02-02T00:00:00Z")
            .await
            .unwrap();

        let doc = get_by_id(&conn, "id-1").await.unwrap().unwrap();
        assert_eq!(doc.path, "/files/novo.md");
        assert_eq!(doc.title, "novo");
        assert_eq!(doc.updated_at, "2026-02-02T00:00:00Z");
    }

    #[tokio::test]
    async fn path_exists_encontra_apenas_path_existente() {
        let conn = test_conn().await;
        insert(&conn, &sample_doc("id-1", "/files/a.md")).await.unwrap();

        assert!(path_exists(&conn, "/files/a.md").await.unwrap());
        assert!(!path_exists(&conn, "/files/b.md").await.unwrap());
    }

    #[tokio::test]
    async fn path_exists_ignora_soft_delete() {
        let conn = test_conn().await;
        insert(&conn, &sample_doc("id-1", "/files/a.md")).await.unwrap();
        soft_delete(&conn, "id-1", "2026-02-02T00:00:00Z").await.unwrap();

        assert!(!path_exists(&conn, "/files/a.md").await.unwrap());
    }
}
