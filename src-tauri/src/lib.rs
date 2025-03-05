mod db;

use db::App;
use structs::{error::StrRes as Res, routine::Routine};
use tauri::{
    State,
    async_runtime::{Mutex, block_on},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn get_state(app: State<'_, Mutex<App>>) -> Res<Option<Routine>> {
    Ok(app.lock().await.routine().map(|r| r.clone()))
}

#[tauri::command]
async fn new_routine(app: State<'_, Mutex<App>>, name: String, routine: Routine) -> Res<i64> {
    let mut lock = app.lock().await;
    let id = lock.set_routine(name, routine).await.map_err(|e|e.to_string())?;
    Ok(id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(block_on(App::get()).unwrap()))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_state,
            new_routine
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
