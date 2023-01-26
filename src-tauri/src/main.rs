#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/commands

#[tauri::command]
fn xdg_open(args: &str) {
    let mut binding = Command::new("xdg-open");
    let command = binding.arg(args);
    match &command.output(){
        Ok(o) => println!("{}",String::from_utf8_lossy(&o.stdout)),
        Err(_) => eprintln!("Failed to open"),
    }
}

#[tauri::command]
fn system_exec(command: &str, args: &str) {
    let mut binding = Command::new(command);
    let command = binding.arg(args);
    match &command.output(){
        Ok(o) => println!("{}",String::from_utf8_lossy(&o.stdout)),
        Err(_) => eprintln!("Failed to execute"),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![xdg_open, system_exec])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
