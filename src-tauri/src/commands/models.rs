use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db::Database;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub name: String,
    pub repo_id: String,
    pub filename: String,
    pub path: String,
    pub size_bytes: Option<i64>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudBackend {
    pub id: i64,
    pub name: String,
    pub endpoint: String,
    pub model_name: String,
    pub is_active: bool,
}

#[tauri::command]
pub fn get_models(
    db: State<'_, Arc<Database>>,
) -> Result<Vec<Model>, String> {
    db.query_map(
        "SELECT id, name, repo_id, filename, path, size_bytes, is_active FROM models ORDER BY name",
        &[] as &[&dyn rusqlite::types::ToSql],
        |row| {
            Ok(Model {
                id: row.get(0)?,
                name: row.get(1)?,
                repo_id: row.get(2)?,
                filename: row.get(3)?,
                path: row.get(4)?,
                size_bytes: row.get(5)?,
                is_active: row.get::<_, i64>(6)? != 0,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_cloud_backends(
    db: State<'_, Arc<Database>>,
) -> Result<Vec<CloudBackend>, String> {
    db.query_map(
        "SELECT id, name, endpoint, model_name, is_active FROM cloud_backends ORDER BY name",
        &[] as &[&dyn rusqlite::types::ToSql],
        |row| {
            Ok(CloudBackend {
                id: row.get(0)?,
                name: row.get(1)?,
                endpoint: row.get(2)?,
                model_name: row.get(3)?,
                is_active: row.get::<_, i64>(4)? != 0,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_cloud_backend(
    db: State<'_, Arc<Database>>,
    name: String,
    endpoint: String,
    api_key: String,
    model_name: String,
) -> Result<CloudBackend, String> {
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    // In a real app, encrypt api_key before storing
    db.execute(
        "INSERT INTO cloud_backends (name, endpoint, api_key_encrypted, model_name, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[&name, &endpoint, &api_key, &model_name, &now],
    ).map_err(|e| e.to_string())?;
    
    let id = db.query_row("SELECT last_insert_rowid()", &[] as &[&dyn rusqlite::types::ToSql], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(CloudBackend {
        id,
        name,
        endpoint,
        model_name,
        is_active: false,
    })
}

#[tauri::command]
pub fn delete_cloud_backend(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    db.execute("DELETE FROM cloud_backends WHERE id = ?1", &[&id.to_string()])
        .map_err(|e| e.to_string())?;
    Ok(())
}
