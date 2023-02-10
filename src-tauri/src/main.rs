#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn on_window_event(event:tauri::GlobalWindowEvent){
    match event.event(){
        tauri::WindowEvent::CloseRequested { api , .. } => {
            api.prevent_close();
            let window = event.window().clone();
            match tauri_api::dialog::ask("确认关闭？", "即将关闭？"){
                tauri_api::dialog::DialogSelection::Yes => window.close().expect("panic!"),
                tauri_api::dialog::DialogSelection::No => (),
                _ => {}
            }
        },
        _ => {},
    }
}

fn main() {
    tauri::Builder::default()
        .on_window_event(on_window_event)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
