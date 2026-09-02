use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db::Database;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub vault_id: Option<i64>,
    pub title: String,
    pub content: String,
    pub tags: String,
    pub metadata: String,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub modified_at: String,
}

#[tauri::command]
pub fn create_note(
    db: State<'_, Arc<Database>>,
    title: String,
    content: String,
    vault_id: Option<i64>,
) -> Result<Note, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let content_hash = format!("{:x}", md5::compute(content.as_bytes()));
    
    db.execute(
        "INSERT INTO notes (vault_id, title, content, content_hash, created_at, modified_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        &[&vault_id as &dyn rusqlite::types::ToSql, &title, &content, &content_hash, &now, &now],
    ).map_err(|e| e.to_string())?;
    
    let id = db.query_row("SELECT last_insert_rowid()", &[] as &[&dyn rusqlite::types::ToSql], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(Note {
        id,
        vault_id,
        title,
        content,
        tags: "[]".to_string(),
        metadata: "{}".to_string(),
        created_at: now.clone(),
        modified_at: now,
    })
}

#[tauri::command]
pub fn get_notes(
    db: State<'_, Arc<Database>>,
    vault_id: Option<i64>,
) -> Result<Vec<Note>, String> {
    let notes = if let Some(vid) = vault_id {
        db.query_map(
            "SELECT id, vault_id, title, content, tags, metadata, created_at, modified_at FROM notes WHERE vault_id = ?1 ORDER BY modified_at DESC",
            &[&vid as &dyn rusqlite::types::ToSql],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    vault_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    tags: row.get(4)?,
                    metadata: row.get(5)?,
                    created_at: row.get(6)?,
                    modified_at: row.get(7)?,
                })
            },
        ).map_err(|e| e.to_string())?
    } else {
        db.query_map(
            "SELECT id, vault_id, title, content, tags, metadata, created_at, modified_at FROM notes ORDER BY modified_at DESC",
            &[] as &[&dyn rusqlite::types::ToSql],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    vault_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    tags: row.get(4)?,
                    metadata: row.get(5)?,
                    created_at: row.get(6)?,
                    modified_at: row.get(7)?,
                })
            },
        ).map_err(|e| e.to_string())?
    };
    Ok(notes)
}

#[tauri::command]
pub fn get_note(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<Note, String> {
    db.query_row(
        "SELECT id, vault_id, title, content, tags, metadata, created_at, modified_at FROM notes WHERE id = ?1",
        &[&id as &dyn rusqlite::types::ToSql],
        |row| {
            Ok(Note {
                id: row.get(0)?,
                vault_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                tags: row.get(4)?,
                metadata: row.get(5)?,
                created_at: row.get(6)?,
                modified_at: row.get(7)?,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_note(
    db: State<'_, Arc<Database>>,
    id: i64,
    title: Option<String>,
    content: Option<String>,
) -> Result<Note, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    let t = title.as_deref().unwrap_or("");
    let c = content.as_deref().unwrap_or("");
    let content_hash = format!("{:x}", md5::compute(c.as_bytes()));
    
    db.execute(
        "UPDATE notes SET title = ?1, content = ?2, content_hash = ?3, modified_at = ?4 WHERE id = ?5",
        &[&t, &c, &content_hash, &now, &id as &dyn rusqlite::types::ToSql],
    ).map_err(|e| e.to_string())?;
    
    get_note(db, id)
}

#[tauri::command]
pub fn delete_note(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    db.execute("DELETE FROM notes WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn create_vault(
    db: State<'_, Arc<Database>>,
    name: String,
    description: Option<String>,
) -> Result<Vault, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    db.execute(
        "INSERT INTO vaults (name, description, created_at, modified_at) VALUES (?1, ?2, ?3, ?4)",
        &[&name as &dyn rusqlite::types::ToSql, &description as &dyn rusqlite::types::ToSql, &now, &now],
    ).map_err(|e| e.to_string())?;
    
    let id = db.query_row("SELECT last_insert_rowid()", &[] as &[&dyn rusqlite::types::ToSql], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(Vault {
        id,
        name,
        description,
        created_at: now.clone(),
        modified_at: now,
    })
}

#[tauri::command]
pub fn get_vaults(
    db: State<'_, Arc<Database>>,
) -> Result<Vec<Vault>, String> {
    db.query_map(
        "SELECT id, name, description, created_at, modified_at FROM vaults ORDER BY name",
        &[] as &[&dyn rusqlite::types::ToSql],
        |row| {
            Ok(Vault {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                modified_at: row.get(4)?,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_vault(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    db.execute("DELETE FROM vaults WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn search_notes(
    db: State<'_, Arc<Database>>,
    query: String,
    vault_id: Option<i64>,
) -> Result<Vec<Note>, String> {
    let search_pattern = format!("%{}%", query);
    
    let notes = if let Some(vid) = vault_id {
        db.query_map(
            "SELECT id, vault_id, title, content, tags, metadata, created_at, modified_at FROM notes WHERE vault_id = ?1 AND (title LIKE ?2 OR content LIKE ?3) ORDER BY modified_at DESC",
            &[&vid as &dyn rusqlite::types::ToSql, &search_pattern, &search_pattern],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    vault_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    tags: row.get(4)?,
                    metadata: row.get(5)?,
                    created_at: row.get(6)?,
                    modified_at: row.get(7)?,
                })
            },
        ).map_err(|e| e.to_string())?
    } else {
        db.query_map(
            "SELECT id, vault_id, title, content, tags, metadata, created_at, modified_at FROM notes WHERE title LIKE ?1 OR content LIKE ?2 ORDER BY modified_at DESC",
            &[&search_pattern as &dyn rusqlite::types::ToSql, &search_pattern],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    vault_id: row.get(1)?,
                    title: row.get(2)?,
                    content: row.get(3)?,
                    tags: row.get(4)?,
                    metadata: row.get(5)?,
                    created_at: row.get(6)?,
                    modified_at: row.get(7)?,
                })
            },
        ).map_err(|e| e.to_string())?
    };
    
    Ok(notes)
}
