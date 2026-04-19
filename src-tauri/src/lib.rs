mod db;
mod entries;
mod error;
mod paths;
mod settings;

use error::LoamResult;
use settings::Settings;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    root: Mutex<PathBuf>,
}

fn resolve_root_from_settings(user_override: Option<&str>) -> LoamResult<PathBuf> {
    let base = paths::default_base()?;
    Ok(paths::resolve_root(
        &base,
        user_override.map(std::path::Path::new),
    ))
}

#[tauri::command]
fn ensure_loam_dir(state: State<AppState>) -> LoamResult<String> {
    let root = state.root.lock().unwrap().clone();
    paths::ensure_dirs(&root)?;
    let conn = db::open(&root)?;
    db::migrate(&conn)?;
    Ok(root.to_string_lossy().to_string())
}

#[tauri::command]
fn write_entry(date: String, body: String, state: State<AppState>) -> LoamResult<String> {
    let root = state.root.lock().unwrap().clone();
    let path = entries::write_entry(&root, &date, &body)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn read_entry(date: String, state: State<AppState>) -> LoamResult<Option<String>> {
    let root = state.root.lock().unwrap().clone();
    entries::read_entry(&root, &date)
}

#[tauri::command]
fn list_entries(state: State<AppState>) -> LoamResult<Vec<String>> {
    let root = state.root.lock().unwrap().clone();
    entries::list_entries(&root)
}

#[tauri::command]
fn load_settings(state: State<AppState>) -> LoamResult<Settings> {
    let root = state.root.lock().unwrap().clone();
    settings::load_settings(&root)
}

#[tauri::command]
fn save_settings(
    new_settings: Settings,
    state: State<AppState>,
) -> LoamResult<()> {
    let mut root_guard = state.root.lock().unwrap();
    settings::save_settings(&root_guard, &new_settings)?;
    if let Some(new_path) = new_settings.loam_path.as_deref() {
        *root_guard = PathBuf::from(new_path);
    } else {
        *root_guard = resolve_root_from_settings(None)?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial_root = resolve_root_from_settings(None)
        .expect("could not resolve initial Loam directory");

    tauri::Builder::default()
        .manage(AppState {
            root: Mutex::new(initial_root),
        })
        .invoke_handler(tauri::generate_handler![
            ensure_loam_dir,
            write_entry,
            read_entry,
            list_entries,
            load_settings,
            save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
