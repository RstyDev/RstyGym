mod db;

use std::sync::Arc;
use db::App;
use tauri::{async_runtime::{block_on, Mutex},State};
use structs::{error::StrRes as Res,routine::Routine};


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn get_state(app: State<'_,Mutex<App>>) -> Res<Option<Routine>> {
    Ok(app.lock().await.routine().map(|r|r.clone()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(block_on(App::get()).unwrap()))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
