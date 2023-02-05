#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{GlobalWindowEvent};



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn window_event(event:GlobalWindowEvent){
    match event.event() {
        // tauri::WindowEvent::Resized(_) => todo!(),
        // tauri::WindowEvent::Moved(_) => todo!(),
        tauri::WindowEvent::CloseRequested { api , .. } => {
            api.prevent_close();
            let window = event.window().clone();
            window.close().expect("Error!");
        },
        // tauri::WindowEvent::Destroyed => todo!(),
        // tauri::WindowEvent::Focused(_) => todo!(),
        // tauri::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size , .. } => todo!(),
        // tauri::WindowEvent::FileDrop(_) => todo!(),
        // tauri::WindowEvent::ThemeChanged(_) => todo!(),
        _ => todo!(),
    }
}

fn main() {
    tauri::Builder::default()
        //.on_window_event(window_event)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
