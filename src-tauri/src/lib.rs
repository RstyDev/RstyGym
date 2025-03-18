mod db;

use db::App;
use structs::{day::Day, error::StrRes as Res, routine::Routine};
use tauri::{
    State,
    async_runtime::{Mutex, block_on},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn check_in(app: State<'_, Mutex<App>>) -> Res<(Day, Option<Day>)> {
    let mut lock = app.lock().await;
    lock.check_in().await.map_err(|e| e.to_string())?;
    let routine = lock.routine().unwrap();
    let today = routine.today().cloned().unwrap();
    let last_day = routine.last_day(&today).cloned();
    Ok((today, last_day))
}
#[tauri::command]
async fn get_state(app: State<'_, Mutex<App>>) -> Res<Option<Routine>> {
    Ok(app.lock().await.routine().map(|r| r.clone()))
}

#[tauri::command]
async fn new_routine(app: State<'_, Mutex<App>>, name: String, routine: Routine) -> Res<i64> {
    let mut lock = app.lock().await;
    let id = lock
        .set_routine(name, routine)
        .await
        .map_err(|e| e.to_string())?;
    Ok(id)
}

#[tauri::command]
async fn  update_weight(app: State<'_, Mutex<App>>, exercise_index: u8, index: u8, weight: f32) -> Res<()> {
    let mut lock = app.lock().await;
    lock.update_weight(exercise_index, index, weight).await.map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(block_on(App::get()).unwrap()))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            check_in,
            get_state,
            new_routine,
            update_weight,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
