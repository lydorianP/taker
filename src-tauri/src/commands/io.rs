use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db::Database;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub format: String, // "markdown", "json", "html"
    pub include_metadata: bool,
}

#[tauri::command]
pub async fn export_note(
    _app: tauri::AppHandle,
    db: State<'_, Arc<Database>>,
    note_id: i64,
    format: Option<String>,
) -> Result<String, String> {
    let note: crate::commands::notes::Note = db.query_row(
        "SELECT id, vault_id, title, content, tags, metadata, created_at, modified_at FROM notes WHERE id = ?1",
        &[&note_id as &dyn rusqlite::types::ToSql],
        |row| {
            Ok(crate::commands::notes::Note {
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
    ).map_err(|e| e.to_string())?;

    let format = format.unwrap_or_else(|| "markdown".to_string());

    match format.as_str() {
        "markdown" => {
            let mut md = format!("# {}\n\n", note.title);
            md.push_str(&note.content);
            Ok(md)
        }
        "json" => {
            serde_json::to_string_pretty(&note).map_err(|e| e.to_string())
        }
        "html" => {
            let html = format!(
                r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{}</title>
    <style>
        body {{ font-family: system-ui, sans-serif; max-width: 800px; margin: 0 auto; padding: 2rem; }}
        h1 {{ color: #2C2825; }}
        .content {{ line-height: 1.6; }}
        .meta {{ color: #6B6560; font-size: 0.875rem; margin-top: 2rem; }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <div class="content">{}</div>
    <div class="meta">
        <p>Created: {}</p>
        <p>Modified: {}</p>
    </div>
</body>
</html>"#,
                note.title, note.title, note.content, note.created_at, note.modified_at
            );
            Ok(html)
        }
        _ => Err(format!("Unsupported format: {}", format)),
    }
}

#[tauri::command]
pub async fn import_note(
    db: State<'_, Arc<Database>>,
    content: String,
    format: Option<String>,
    vault_id: Option<i64>,
) -> Result<crate::commands::notes::Note, String> {
    let format = format.unwrap_or_else(|| "markdown".to_string());

    let (title, note_content) = match format.as_str() {
        "markdown" | "md" => {
            // Extract title from first heading or use first line
            let lines: Vec<&str> = content.lines().collect();
            let title = lines.first()
                .and_then(|l| l.strip_prefix("# "))
                .unwrap_or("Imported Note")
                .to_string();
            let content = lines.into_iter().skip(1).collect::<Vec<_>>().join("\n").trim().to_string();
            (title, content)
        }
        "json" => {
            let note: crate::commands::notes::Note = serde_json::from_str(&content)
                .map_err(|e| format!("Invalid JSON: {}", e))?;
            (note.title, note.content)
        }
        _ => {
            ("Imported Note".to_string(), content)
        }
    };

    // Create the note
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let content_hash = format!("{:x}", md5::compute(note_content.as_bytes()));
    
    db.execute(
        "INSERT INTO notes (vault_id, title, content, content_hash, created_at, modified_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        &[&vault_id as &dyn rusqlite::types::ToSql, &title, &note_content, &content_hash, &now, &now],
    ).map_err(|e| e.to_string())?;
    
    let id = db.query_row("SELECT last_insert_rowid()", &[] as &[&dyn rusqlite::types::ToSql], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    Ok(crate::commands::notes::Note {
        id,
        vault_id,
        title,
        content: note_content,
        tags: "[]".to_string(),
        metadata: "{}".to_string(),
        created_at: now.clone(),
        modified_at: now,
    })
}

#[tauri::command]
pub async fn export_vault(
    db: State<'_, Arc<Database>>,
    vault_id: i64,
    _format: Option<String>,
) -> Result<Vec<crate::commands::notes::Note>, String> {
    let notes = db.query_map(
        "SELECT id, vault_id, title, content, tags, metadata, created_at, modified_at FROM notes WHERE vault_id = ?1 ORDER BY title",
        &[&vault_id as &dyn rusqlite::types::ToSql],
        |row| {
            Ok(crate::commands::notes::Note {
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
    ).map_err(|e| e.to_string())?;

    Ok(notes)
}
