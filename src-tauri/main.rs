#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Stdio};
use std::io::Write;
use tauri::command;

#[command]
fn compute_square(number: i32) -> Result<i32, String> {
    let mut child = Command::new("python")
        .arg("python/backend.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    let stdin = child.stdin.as_mut().ok_or("stdin error")?;
    let input = serde_json::json!({ "number": number }).to_string();
    stdin.write_all(input.as_bytes()).map_err(|e| e.to_string())?;

    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(&stdout).map_err(|e| e.to_string())?;
    Ok(value["squared"].as_i64().unwrap_or(0) as i32)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compute_square])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
