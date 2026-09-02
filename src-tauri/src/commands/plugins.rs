use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db::Database;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub manifest_path: String,
    pub is_enabled: bool,
}

#[tauri::command]
pub fn get_plugins(
    db: State<'_, Arc<Database>>,
) -> Result<Vec<Plugin>, String> {
    db.query_map(
        "SELECT id, name, version, manifest_path, is_enabled FROM plugins ORDER BY name",
        &[] as &[&dyn rusqlite::types::ToSql],
        |row| {
            Ok(Plugin {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                manifest_path: row.get(3)?,
                is_enabled: row.get::<_, i64>(4)? != 0,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn enable_plugin(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    db.execute("UPDATE plugins SET is_enabled = 1 WHERE id = ?1", &[&id.to_string()])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn disable_plugin(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    db.execute("UPDATE plugins SET is_enabled = 0 WHERE id = ?1", &[&id.to_string()])
        .map_err(|e| e.to_string())?;
    Ok(())
}
