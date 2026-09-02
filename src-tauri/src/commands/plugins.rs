use serde::{Deserialize, Serialize};
use tauri::{State, Manager};
use crate::db::Database;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub manifest_path: String,
    pub is_enabled: bool,
    pub description: Option<String>,
    pub author: Option<String>,
    pub plugin_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMarketplace {
    pub plugins: Vec<PluginInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub plugin_type: String,
    pub downloads: i64,
    pub rating: f64,
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
                description: None,
                author: None,
                plugin_type: None,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn enable_plugin(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    db.execute("UPDATE plugins SET is_enabled = 1 WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn disable_plugin(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    db.execute("UPDATE plugins SET is_enabled = 0 WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn install_plugin(
    app: tauri::AppHandle,
    db: State<'_, Arc<Database>>,
    plugin_id: String,
    name: String,
    version: String,
    description: String,
    author: String,
    plugin_type: String,
) -> Result<Plugin, String> {
    // Create plugins directory
    let plugins_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("plugins");
    
    std::fs::create_dir_all(&plugins_dir)
        .map_err(|e| format!("Failed to create plugins directory: {}", e))?;
    
    let plugin_dir = plugins_dir.join(&plugin_id);
    std::fs::create_dir_all(&plugin_dir)
        .map_err(|e| format!("Failed to create plugin directory: {}", e))?;
    
    // Create manifest.toml
    let manifest = format!(
        r#"[plugin]
id = "{}"
name = "{}"
version = "{}"
description = "{}"
author = "{}"
type = "{}"
min_app_version = "0.1.0"

[plugin.permissions]
requires = []
optional = []
"#,
        plugin_id, name, version, description, author, plugin_type
    );
    
    std::fs::write(plugin_dir.join("manifest.toml"), manifest)
        .map_err(|e| format!("Failed to write manifest: {}", e))?;
    
    // Save to database
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    db.execute(
        "INSERT INTO plugins (manifest_path, name, version, is_enabled, installed_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        &[
            &plugin_dir.to_str().unwrap_or("") as &dyn rusqlite::types::ToSql,
            &name,
            &version,
            &1i64,
            &now,
        ],
    ).map_err(|e| e.to_string())?;
    
    let id = db.query_row("SELECT last_insert_rowid()", &[] as &[&dyn rusqlite::types::ToSql], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    
    Ok(Plugin {
        id,
        name,
        version,
        manifest_path: plugin_dir.to_str().unwrap_or("").to_string(),
        is_enabled: true,
        description: Some(description),
        author: Some(author),
        plugin_type: Some(plugin_type),
    })
}

#[tauri::command]
pub fn uninstall_plugin(
    db: State<'_, Arc<Database>>,
    id: i64,
) -> Result<(), String> {
    // Get plugin info first
    let plugin: Plugin = db.query_row(
        "SELECT id, name, version, manifest_path, is_enabled FROM plugins WHERE id = ?1",
        &[&id as &dyn rusqlite::types::ToSql],
        |row| {
            Ok(Plugin {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                manifest_path: row.get(3)?,
                is_enabled: row.get::<_, i64>(4)? != 0,
                description: None,
                author: None,
                plugin_type: None,
            })
        },
    ).map_err(|e| e.to_string())?;
    
    // Delete plugin directory
    let _ = std::fs::remove_dir_all(&plugin.manifest_path);
    
    // Delete from database
    db.execute("DELETE FROM plugins WHERE id = ?1", &[&id as &dyn rusqlite::types::ToSql])
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_marketplace_plugins() -> Result<Vec<PluginInfo>, String> {
    // Placeholder: In production, fetch from Taker plugin registry
    // For now, return example plugins
    Ok(vec![
        PluginInfo {
            id: "theme-dark".to_string(),
            name: "Dark Theme".to_string(),
            version: "1.0.0".to_string(),
            description: "A beautiful dark theme for Taker".to_string(),
            author: "Taker Team".to_string(),
            plugin_type: "theme".to_string(),
            downloads: 1250,
            rating: 4.8,
        },
        PluginInfo {
            id: "duolingo-format".to_string(),
            name: "Duolingo Flashcard Format".to_string(),
            version: "1.0.0".to_string(),
            description: "Generate flashcards in Duolingo-style format".to_string(),
            author: "Community".to_string(),
            plugin_type: "output".to_string(),
            downloads: 890,
            rating: 4.5,
        },
        PluginInfo {
            id: "anki-export".to_string(),
            name: "Anki Export".to_string(),
            version: "1.0.0".to_string(),
            description: "Export flashcards to Anki format".to_string(),
            author: "Community".to_string(),
            plugin_type: "integration".to_string(),
            downloads: 2100,
            rating: 4.9,
        },
    ])
}
