use serde::{Deserialize, Serialize};
use tauri::{State, Manager};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceModel {
    pub id: String,
    pub name: String,
    pub downloads: i64,
    pub likes: i64,
    pub pipeline_tag: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub model_id: String,
    pub progress: f32,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
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

#[tauri::command]
pub async fn search_huggingface(
    query: String,
    limit: Option<u32>,
) -> Result<Vec<HuggingFaceModel>, String> {
    let limit = limit.unwrap_or(10);
    let url = format!(
        "https://huggingface.co/api/models?search={}&limit={}&filter=gguf",
        urlencoding::encode(&query),
        limit
    );
    
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to search HuggingFace: {}", e))?;
    
    let models: Vec<serde_json::Value> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    let results = models
        .into_iter()
        .map(|m| HuggingFaceModel {
            id: m["id"].as_str().unwrap_or("").to_string(),
            name: m["id"].as_str().unwrap_or("").to_string(),
            downloads: m["downloads"].as_i64().unwrap_or(0),
            likes: m["likes"].as_i64().unwrap_or(0),
            pipeline_tag: m["pipeline_tag"].as_str().map(|s| s.to_string()),
            tags: m["tags"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|t| t.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        })
        .collect();
    
    Ok(results)
}

#[tauri::command]
pub async fn download_model(
    app: tauri::AppHandle,
    db: State<'_, Arc<Database>>,
    repo_id: String,
    filename: String,
) -> Result<Model, String> {
    use std::path::PathBuf;
    
    // Get model directory
    let model_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("models");
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&model_dir)
        .map_err(|e| format!("Failed to create model directory: {}", e))?;
    
    let file_path = model_dir.join(&filename);
    
    // Download from HuggingFace
    let url = format!(
        "https://huggingface.co/{}/resolve/main/{}",
        repo_id,
        filename
    );
    
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to download model: {}", e))?;
    
    // Stream to file
    let mut file = std::fs::File::create(&file_path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Download error: {}", e))?;
        std::io::Write::write_all(&mut file, &chunk)
            .map_err(|e| format!("Failed to write file: {}", e))?;
    }
    
    let size_bytes = std::fs::metadata(&file_path)
        .map(|m| m.len() as i64)
        .ok();
    
    // Save to database
    let name = repo_id.split('/').last().unwrap_or(&filename).to_string();
    
    db.execute(
        "INSERT INTO models (name, repo_id, filename, path, size_bytes, is_active) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        &[&name, &repo_id, &filename, &file_path.to_str().unwrap_or(""), &size_bytes, &0i64],
    ).map_err(|e| e.to_string())?;
    
    let id = db.query_row("SELECT last_insert_rowid()", &[] as &[&dyn rusqlite::types::ToSql], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(Model {
        id,
        name,
        repo_id,
        filename,
        path: file_path.to_str().unwrap_or("").to_string(),
        size_bytes,
        is_active: false,
    })
}

#[tauri::command]
pub fn delete_model(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    // Get model path before deleting
    let model: Model = db.query_row(
        "SELECT id, name, repo_id, filename, path, size_bytes, is_active FROM models WHERE id = ?1",
        &[&id as &dyn rusqlite::types::ToSql],
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
    ).map_err(|e| e.to_string())?;
    
    // Delete file
    let _ = std::fs::remove_file(&model.path);
    
    // Delete from database
    db.execute("DELETE FROM models WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
