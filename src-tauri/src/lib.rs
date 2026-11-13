// Question Desk — scaffold only.
//
// Build your Tauri commands here (M2: save_question, list_questions,
// delete_question — M4: draft_answer). Register each one in the
// invoke_handler below as you add it. See:
// https://tauri.app/develop/calling-rust/

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Clone)]
struct Question {
    id: String,
    asked_at: String,
    asker: String,
    context: String,
    question: String,
    tags: Vec<String>,
    draft: Option<serde_json::Value>,
}

fn store_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_data_dir().map_err(|e| e.to_string())?;

    fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    path.push("questions.json");

    Ok(path)
}

fn load_questions(path: &PathBuf) -> Result<Vec<Question>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;

    let questions: Vec<Question> = serde_json::from_str(&contents).map_err(|e| e.to_string())?;

    Ok(questions)
}

fn save_questions(path: &PathBuf, questions: &Vec<Question>) -> Result<(), String> {
    let contents = serde_json::to_string_pretty(questions).map_err(|e| e.to_string())?;

    fs::write(path, contents).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn save_question(
    app: tauri::AppHandle,
    asker: String,
    context: String,
    question: String,
    tags: Vec<String>,
) -> Result<Question, String> {
    let path = store_path(&app)?;
    let mut questions = load_questions(&path)?;

    let new_question = Question {
        id: format!("q_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S")),
        asked_at: chrono::Utc::now().to_rfc3339(),
        asker,
        context,
        question,
        tags,
        draft: None,
    };

    questions.push(new_question.clone());
    save_questions(&path, &questions)?;

    Ok(new_question)
}

#[tauri::command]
fn list_questions(app: tauri::AppHandle) -> Result<Vec<Question>, String> {
    let path = store_path(&app)?;
    let mut questions = load_questions(&path)?;

    questions.sort_by(|a, b| b.asked_at.cmp(&a.asked_at));

    Ok(questions)
}

#[tauri::command]
fn delete_question(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let path = store_path(&app)?;
    let mut questions = load_questions(&path)?;

    let original_length = questions.len();
    questions.retain(|q| q.id != id);

    if questions.len() == original_length {
        return Err("Question not found".to_string());
    }

    save_questions(&path, &questions)?;

    Ok(())
}

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
