use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db::Database;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    pub value: String,
}

#[tauri::command]
pub fn get_setting(
    db: State<'_, Arc<Database>>,
    key: String,
) -> Result<Option<String>, String> {
    let result = db.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        &[&key],
        |row| row.get(0),
    );
    
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn set_setting(
    db: State<'_, Arc<Database>>,
    key: String,
    value: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    db.execute(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
        &[&key, &value, &now],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub fn get_all_settings(
    db: State<'_, Arc<Database>>,
) -> Result<Vec<Setting>, String> {
    db.query_map(
        "SELECT key, value FROM settings ORDER BY key",
        &[] as &[&dyn rusqlite::types::ToSql],
        |row| {
            Ok(Setting {
                key: row.get(0)?,
                value: row.get(1)?,
            })
        },
    ).map_err(|e| e.to_string())
}
