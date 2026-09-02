use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db::Database;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub note_id: i64,
    pub summary: String,
    pub key_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: Option<i64>,
    pub note_id: i64,
    pub question: String,
    pub answer: String,
    pub hint: Option<String>,
    pub difficulty: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slideshow {
    pub id: Option<i64>,
    pub note_id: i64,
    pub title: String,
    pub slides: Vec<Slide>,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub title: String,
    pub content: String,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn summarize_note(
    db: State<'_, Arc<Database>>,
    note_id: i64,
) -> Result<Summary, String> {
    // Get note content
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

    // Simple extractive summarization (placeholder for LLM)
    let summary = generate_summary(&note.content);
    let key_points = extract_key_points(&note.content);

    Ok(Summary {
        note_id,
        summary,
        key_points,
    })
}

#[tauri::command]
pub async fn generate_flashcards(
    db: State<'_, Arc<Database>>,
    note_id: i64,
) -> Result<Vec<Flashcard>, String> {
    // Get note content
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

    // Generate flashcards from content
    let flashcards = create_flashcards_from_content(&note.content, note_id);

    // Save to database
    for card in &flashcards {
        db.execute(
            "INSERT INTO flashcards (note_id, question, answer, hint, difficulty) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[
                &note_id as &dyn rusqlite::types::ToSql,
                &card.question,
                &card.answer,
                &card.hint,
                &card.difficulty,
            ],
        ).map_err(|e| e.to_string())?;
    }

    Ok(flashcards)
}

#[tauri::command]
pub async fn generate_slideshow(
    db: State<'_, Arc<Database>>,
    note_id: i64,
) -> Result<Slideshow, String> {
    // Get note content
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

    // Generate slides from content
    let slides = create_slides_from_content(&note.content);
    let title = format!("{} - Slideshow", note.title);

    let slideshow = Slideshow {
        id: None,
        note_id,
        title: title.clone(),
        slides,
        format: "revealjs".to_string(),
    };

    // Save to database
    let slides_json = serde_json::to_string(&slideshow.slides).map_err(|e| e.to_string())?;
    db.execute(
        "INSERT INTO slideshows (note_id, title, slides_json, format) VALUES (?1, ?2, ?3, ?4)",
        &[
            &note_id as &dyn rusqlite::types::ToSql,
            &title,
            &slides_json,
            &slideshow.format,
        ],
    ).map_err(|e| e.to_string())?;

    let id = db.query_row("SELECT last_insert_rowid()", &[] as &[&dyn rusqlite::types::ToSql], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    Ok(Slideshow {
        id: Some(id),
        ..slideshow
    })
}

#[tauri::command]
pub async fn get_flashcards(
    db: State<'_, Arc<Database>>,
    note_id: i64,
) -> Result<Vec<Flashcard>, String> {
    db.query_map(
        "SELECT id, note_id, question, answer, hint, difficulty FROM flashcards WHERE note_id = ?1 ORDER BY created_at DESC",
        &[&note_id as &dyn rusqlite::types::ToSql],
        |row| {
            Ok(Flashcard {
                id: row.get(0)?,
                note_id: row.get(1)?,
                question: row.get(2)?,
                answer: row.get(3)?,
                hint: row.get(4)?,
                difficulty: row.get(5)?,
            })
        },
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_slideshows(
    db: State<'_, Arc<Database>>,
    note_id: i64,
) -> Result<Vec<Slideshow>, String> {
    let slideshows: Vec<Slideshow> = db.query_map(
        "SELECT id, note_id, title, slides_json, format FROM slideshows WHERE note_id = ?1 ORDER BY created_at DESC",
        &[&note_id as &dyn rusqlite::types::ToSql],
        |row| {
            let slides_json: String = row.get(3)?;
            let slides: Vec<Slide> = serde_json::from_str(&slides_json).unwrap_or_default();
            Ok(Slideshow {
                id: row.get(0)?,
                note_id: row.get(1)?,
                title: row.get(2)?,
                slides,
                format: row.get(4)?,
            })
        },
    ).map_err(|e| e.to_string())?;

    Ok(slideshows)
}

// Helper functions (placeholder implementations - would use LLM in production)

fn generate_summary(content: &str) -> String {
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.is_empty() {
        return "Empty note".to_string();
    }
    
    // Simple extractive summary - take first few sentences
    let summary_lines: Vec<&str> = lines.into_iter().take(3).collect();
    summary_lines.join(" ")
}

fn extract_key_points(content: &str) -> Vec<String> {
    let mut key_points = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        // Look for bullet points or numbered lists
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("• ") {
            key_points.push(trimmed[2..].to_string());
        } else if trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) && trimmed.contains('.') {
            if let Some(pos) = trimmed.find('.') {
                key_points.push(trimmed[pos + 1..].trim().to_string());
            }
        }
        // Look for headers
        else if trimmed.starts_with('#') {
            key_points.push(trimmed.trim_start_matches('#').trim().to_string());
        }
        
        if key_points.len() >= 5 {
            break;
        }
    }
    
    if key_points.is_empty() {
        // Fallback: take first sentence
        if let Some(first_line) = content.lines().next() {
            key_points.push(first_line.trim().to_string());
        }
    }
    
    key_points
}

fn create_flashcards_from_content(content: &str, note_id: i64) -> Vec<Flashcard> {
    let mut flashcards = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Simple flashcard generation based on structure
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Create flashcard from headers
        if trimmed.starts_with('#') {
            let question = format!("What is discussed in: {}?", trimmed.trim_start_matches('#').trim());
            let answer = get_context_after(&lines, i);
            flashcards.push(Flashcard {
                id: None,
                note_id,
                question,
                answer,
                hint: None,
                difficulty: 0,
            });
        }
        
        // Create flashcard from definitions (lines with colons)
        else if trimmed.contains(':') && !trimmed.starts_with('#') {
            let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
            if parts.len() == 2 {
                flashcards.push(Flashcard {
                    id: None,
                    note_id,
                    question: format!("Define: {}", parts[0].trim()),
                    answer: parts[1].trim().to_string(),
                    hint: None,
                    difficulty: 0,
                });
            }
        }
        
        if flashcards.len() >= 10 {
            break;
        }
    }
    
    // If no structured content, create basic Q&A pairs
    if flashcards.is_empty() && !lines.is_empty() {
        for chunk in lines.chunks(2) {
            if chunk.len() >= 2 {
                flashcards.push(Flashcard {
                    id: None,
                    note_id,
                    question: chunk[0].trim().to_string(),
                    answer: chunk[1].trim().to_string(),
                    hint: None,
                    difficulty: 0,
                });
            }
        }
    }
    
    flashcards
}

fn get_context_after(lines: &[&str], start_idx: usize) -> String {
    let mut context = Vec::new();
    for i in (start_idx + 1)..lines.len() {
        let line = lines[i].trim();
        if line.is_empty() || line.starts_with('#') {
            break;
        }
        context.push(line);
        if context.len() >= 3 {
            break;
        }
    }
    context.join(" ")
}

fn create_slides_from_content(content: &str) -> Vec<Slide> {
    let mut slides = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Title slide
    if let Some(first_line) = lines.first() {
        let title = first_line.trim().trim_start_matches('#').trim();
        slides.push(Slide {
            title: title.to_string(),
            content: String::new(),
            notes: None,
        });
    }
    
    // Create slides from sections
    let mut current_slide: Option<Slide> = None;
    
    for line in &lines {
        let trimmed = line.trim();
        
        if trimmed.starts_with('#') {
            // Save previous slide
            if let Some(slide) = current_slide.take() {
                slides.push(slide);
            }
            
            // Start new slide
            current_slide = Some(Slide {
                title: trimmed.trim_start_matches('#').trim().to_string(),
                content: String::new(),
                notes: None,
            });
        } else if let Some(ref mut slide) = current_slide {
            if !trimmed.is_empty() {
                if !slide.content.is_empty() {
                    slide.content.push('\n');
                }
                slide.content.push_str(trimmed);
            }
        }
    }
    
    // Add final slide
    if let Some(slide) = current_slide {
        slides.push(slide);
    }
    
    // If no sections found, create slides from paragraphs
    if slides.len() <= 1 {
        let paragraphs: Vec<&str> = content.split("\n\n").collect();
        for (i, paragraph) in paragraphs.iter().enumerate() {
            let trimmed = paragraph.trim();
            if !trimmed.is_empty() {
                slides.push(Slide {
                    title: if i == 0 { "Introduction".to_string() } else { format!("Section {}", i) },
                    content: trimmed.to_string(),
                    notes: None,
                });
            }
        }
    }
    
    slides
}
