// Question Desk — scaffold only.
//
// Build your Tauri commands here (M2: save_question, list_questions,
// delete_question — M4: draft_answer). Register each one in the
// invoke_handler below as you add it. See:
// https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_question,
            list_questions,
            delete_question
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone)]
struct Question {
    id: String,
    asked_at: String,
    asker: String,
    context: String,
    question: String,
    tags: Vec<String>,
    draft: Option<Draft>,

}
#[derive(Serialize, Deserialize, Clone)]
struct Draft {}
fn store_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    fs::create_dir_all(&dir)
        .map_err(|e| e.to_string())?;

    Ok(dir.join("questions.json"))
}fn read_questions(
    path: &PathBuf,
) -> Result<Vec<Question>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path)
        .map_err(|e| e.to_string())?;

    serde_json::from_str(&contents)
        .map_err(|e| e.to_string())
}

fn write_questions(
    path: &PathBuf,
    questions: &[Question],
) -> Result<(), String> {
    let contents = serde_json::to_string_pretty(questions)
        .map_err(|e| e.to_string())?;

    fs::write(path, contents)
        .map_err(|e| e.to_string())
}#[tauri::command]
fn save_question(
    app: AppHandle,
    asker: String,
    context: String,
    question: String,
    tags: Vec<String>,
) -> Result<Question, String> {
    let path = store_path(&app)?;
    let mut questions = read_questions(&path)?;

    let now = chrono::Utc::now();

    let record = Question {
        id: format!("q_{}", now.format("%Y%m%d_%H%M%S")),
        asked_at: now.to_rfc3339(),
        asker,
        context,
        question,
        tags,
        draft: None,
    };

    questions.push(record.clone());

    write_questions(&path, &questions)?;

    Ok(record)
    
}#[tauri::command]
fn list_questions(
    app: AppHandle,
) -> Result<Vec<Question>, String> {
    let path = store_path(&app)?;
    let mut questions = read_questions(&path)?;

    questions.sort_by(|a, b| {
        b.asked_at.cmp(&a.asked_at)
    });

    Ok(questions)
}
#[tauri::command]
fn delete_question(
    app: AppHandle,
    id: String,
) -> Result<(), String> {
    let path = store_path(&app)?;
    let mut questions = read_questions(&path)?;

    let original_len = questions.len();

    questions.retain(|q| q.id != id);

    if questions.len() == original_len {
        return Err("Question not found".to_string());
    }

    write_questions(&path, &questions)?;

    Ok(())
}