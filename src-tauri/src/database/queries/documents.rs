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
