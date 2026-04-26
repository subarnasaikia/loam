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

fn validate_loam_path(p: &str) -> LoamResult<PathBuf> {
    let path = PathBuf::from(p);
    if !path.is_absolute() {
        return Err(error::LoamError::Path("loam_path must be an absolute path".into()));
    }
    // Reject paths that would escape user-space on macOS/Linux.
    let blocked = ["/etc", "/System", "/usr", "/bin", "/sbin", "/lib", "/proc"];
    for prefix in &blocked {
        if path.starts_with(prefix) {
            return Err(error::LoamError::Path(format!("loam_path may not be under {prefix}")));
        }
    }
    Ok(path)
}

#[tauri::command]
fn ensure_loam_dir(state: State<AppState>) -> LoamResult<String> {
    let root = state.root.lock().map_err(|_| error::LoamError::Path("state lock poisoned".into()))?.clone();
    paths::ensure_dirs(&root)?;
    let conn = db::open(&root)?;
    db::migrate(&conn)?;
    Ok(root.to_string_lossy().to_string())
}

#[tauri::command]
fn write_entry(date: String, body: String, state: State<AppState>) -> LoamResult<String> {
    let root = state.root.lock().map_err(|_| error::LoamError::Path("state lock poisoned".into()))?.clone();
    let path = entries::write_entry(&root, &date, &body)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
fn read_entry(date: String, state: State<AppState>) -> LoamResult<Option<String>> {
    let root = state.root.lock().map_err(|_| error::LoamError::Path("state lock poisoned".into()))?.clone();
    entries::read_entry(&root, &date)
}

#[tauri::command]
fn list_entries(state: State<AppState>) -> LoamResult<Vec<String>> {
    let root = state.root.lock().map_err(|_| error::LoamError::Path("state lock poisoned".into()))?.clone();
    entries::list_entries(&root)
}

#[tauri::command]
fn load_settings(state: State<AppState>) -> LoamResult<Settings> {
    let root = state.root.lock().map_err(|_| error::LoamError::Path("state lock poisoned".into()))?.clone();
    settings::load_settings(&root)
}

#[tauri::command]
fn save_settings(
    new_settings: Settings,
    state: State<AppState>,
) -> LoamResult<()> {
    let mut root_guard = state.root.lock().map_err(|_| error::LoamError::Path("state lock poisoned".into()))?;
    settings::save_settings(&root_guard, &new_settings)?;
    if let Some(new_path) = new_settings.loam_path.as_deref() {
        *root_guard = validate_loam_path(new_path)?;
    } else {
        *root_guard = resolve_root_from_settings(None)?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let default_root = resolve_root_from_settings(None)
        .expect("could not resolve initial Loam directory");
    // Honor a previously saved loam_path so custom locations survive restarts.
    let initial_root = settings::load_settings(&default_root)
        .ok()
        .and_then(|s| s.loam_path)
        .and_then(|p| validate_loam_path(&p).ok())
        .unwrap_or(default_root);

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
