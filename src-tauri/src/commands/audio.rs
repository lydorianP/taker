use serde::{Deserialize, Serialize};
use tauri::{State, Manager};
use crate::db::Database;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFile {
    pub id: i64,
    pub note_id: Option<i64>,
    pub filename: String,
    pub path: String,
    pub duration_secs: Option<f64>,
    pub file_size: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    pub text: String,
    pub segments: Vec<TranscriptionSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodcastEpisode {
    pub id: Option<i64>,
    pub title: String,
    pub description: String,
    pub note_ids: Vec<i64>,
    pub audio_path: Option<String>,
    pub duration_secs: Option<f64>,
    pub created_at: String,
}

#[tauri::command]
pub async fn text_to_speech(
    app: tauri::AppHandle,
    text: String,
    _voice: Option<String>,
) -> Result<AudioFile, String> {
    use std::path::PathBuf;
    
    // Create audio directory
    let audio_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("audio");
    
    std::fs::create_dir_all(&audio_dir)
        .map_err(|e| format!("Failed to create audio directory: {}", e))?;
    
    let filename = format!("tts_{}.mp3", chrono::Utc::now().timestamp());
    let file_path = audio_dir.join(&filename);
    
    // Placeholder: In production, use Piper TTS or similar
    // For now, create a placeholder file
    std::fs::write(&file_path, b"placeholder audio")
        .map_err(|e| format!("Failed to create audio file: {}", e))?;
    
    let metadata = std::fs::metadata(&file_path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?;
    
    Ok(AudioFile {
        id: 0,
        note_id: None,
        filename,
        path: file_path.to_str().unwrap_or("").to_string(),
        duration_secs: Some(text.len() as f64 / 15.0), // Rough estimate
        file_size: Some(metadata.len() as i64),
        created_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    })
}

#[tauri::command]
pub async fn transcribe_audio(
    _app: tauri::AppHandle,
    _audio_path: String,
) -> Result<Transcription, String> {
    // Placeholder: In production, use whisper.cpp
    // For now, return placeholder transcription
    Ok(Transcription {
        text: "Transcription would appear here when whisper.cpp is integrated.".to_string(),
        segments: vec![
            TranscriptionSegment {
                start: 0.0,
                end: 2.0,
                text: "Transcription would appear here".to_string(),
            },
            TranscriptionSegment {
                start: 2.0,
                end: 4.0,
                text: "when whisper.cpp is integrated.".to_string(),
            },
        ],
    })
}

#[tauri::command]
pub async fn generate_podcast(
    app: tauri::AppHandle,
    db: State<'_, Arc<Database>>,
    note_ids: Vec<i64>,
    title: Option<String>,
) -> Result<PodcastEpisode, String> {
    use std::path::PathBuf;
    
    // Get notes content
    let mut notes_content = Vec::new();
    for note_id in &note_ids {
        let note: crate::commands::notes::Note = db.query_row(
            "SELECT id, vault_id, title, content, tags, metadata, created_at, modified_at FROM notes WHERE id = ?1",
            &[note_id as &dyn rusqlite::types::ToSql],
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
        notes_content.push(note);
    }
    
    // Create audio directory
    let audio_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("audio")
        .join("podcasts");
    
    std::fs::create_dir_all(&audio_dir)
        .map_err(|e| format!("Failed to create podcast directory: {}", e))?;
    
    let podcast_title = title.unwrap_or_else(|| {
        if let Some(first_note) = notes_content.first() {
            format!("{} and others", first_note.title)
        } else {
            "Untitled Podcast".to_string()
        }
    });
    
    let filename = format!("podcast_{}.mp3", chrono::Utc::now().timestamp());
    let file_path = audio_dir.join(&filename);
    
    // Placeholder: In production, combine TTS outputs
    std::fs::write(&file_path, b"placeholder podcast audio")
        .map_err(|e| format!("Failed to create podcast file: {}", e))?;
    
    let total_text_len: usize = notes_content.iter().map(|n| n.content.len()).sum();
    
    let episode = PodcastEpisode {
        id: None,
        title: podcast_title.clone(),
        description: format!("Podcast generated from {} notes", note_ids.len()),
        note_ids,
        audio_path: Some(file_path.to_str().unwrap_or("").to_string()),
        duration_secs: Some(total_text_len as f64 / 15.0),
        created_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    };
    
    Ok(episode)
}

#[tauri::command]
pub async fn get_audio_files(
    app: tauri::AppHandle,
    note_id: Option<i64>,
) -> Result<Vec<AudioFile>, String> {
    let audio_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("audio");
    
    let mut files = Vec::new();
    
    if audio_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&audio_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "mp3" || ext == "wav" || ext == "ogg") {
                    let metadata = std::fs::metadata(&path)
                        .map_err(|e| format!("Failed to get metadata: {}", e))?;
                    
                    files.push(AudioFile {
                        id: files.len() as i64,
                        note_id,
                        filename: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                        path: path.to_str().unwrap_or("").to_string(),
                        duration_secs: None,
                        file_size: Some(metadata.len() as i64),
                        created_at: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                    });
                }
            }
        }
    }
    
    Ok(files)
}
