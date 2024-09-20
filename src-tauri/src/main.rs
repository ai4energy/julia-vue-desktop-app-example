// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::process::{Command, Child};
use std::sync::Mutex;

struct JuliaProcessState {
    process: Mutex<Option<Child>>,
}

impl JuliaProcessState {
    fn new() -> Self {
        JuliaProcessState {
            process: Mutex::new(None),
        }
    }
}

#[tauri::command]
async fn start_julia_service(state: tauri::State<'_, JuliaProcessState>) -> Result<(), String> {
    // 锁定 Mutex 以检查 Julia 服务状态
    let mut julia_process = state.process.lock().unwrap();

    if julia_process.is_none() {
        // 在当前线程中创建进程，然后异步运行它
        let child = Command::new("julia")
            .arg("--project=./src-julia")
            .arg("src-julia/julia-server.jl")
            .spawn()
            .expect("failed to start Julia service");

        // 将进程存储在 Mutex 中，防止跨线程问题
        *julia_process = Some(child);

        Ok(())
    } else {
        Err("Julia service is already running.".into())
    }
}

#[tauri::command]
async fn stop_julia_service(state: tauri::State<'_, JuliaProcessState>) -> Result<(), String> {
    let mut julia_process = state.process.lock().unwrap();

    if let Some(mut child) = julia_process.take() {
        match child.kill() {
            Ok(_) => {
                println!("Julia service stopped successfully.");
                Ok(())
            }
            Err(e) => {
                println!("Failed to stop Julia service: {:?}", e);
                Err("Failed to stop Julia service.".into())
            }
        }
    } else {
        Err("No Julia service is running.".into())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(JuliaProcessState::new())
        .invoke_handler(tauri::generate_handler![greet, start_julia_service, stop_julia_service])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}