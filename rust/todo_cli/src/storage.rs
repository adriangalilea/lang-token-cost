use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;

use crate::error::AppError;
use crate::models::TodoStore;

fn db_path() -> PathBuf {
    if let Some(dirs) = ProjectDirs::from("", "", "todo-cli") {
        let data_dir = dirs.data_dir();
        fs::create_dir_all(data_dir).expect("failed to create data directory");
        data_dir.join("todos.json")
    } else {
        PathBuf::from("todos.json")
    }
}

pub fn load() -> Result<TodoStore, AppError> {
    let path = db_path();
    if !path.exists() {
        return Ok(TodoStore::default());
    }
    let data = fs::read_to_string(&path)?;
    let store: TodoStore = serde_json::from_str(&data)?;
    Ok(store)
}

pub fn save(store: &TodoStore) -> Result<(), AppError> {
    let path = db_path();
    let data = serde_json::to_string_pretty(store)?;
    fs::write(&path, data)?;
    Ok(())
}
