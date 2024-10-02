#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;
use meval;
fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![calculate])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[command]
fn calculate(expression: String) -> String {
  match meval::eval_str(&expression) {
    Ok(result) => result.to_string(),
    Err(_) => "Error".to_string(),
  }
}