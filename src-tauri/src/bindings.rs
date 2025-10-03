use crate::runtime;

use crate::{cmd_blocking, utils::CommandResult};

#[tauri::command]
pub async fn get_library() -> CommandResult<String> {
    cmd_blocking!(runtime::get_library())
}

#[tauri::command]
pub async fn decrypt(
    key_path: String,
    file_path: String,
    target_path: String,
) -> CommandResult<()> {
    cmd_blocking!(runtime::decrypt(key_path, file_path, target_path))
}

#[tauri::command]
pub async fn get_temp_book_path(
    book_id: String,
    owner_id: String,
    format: String,
) -> CommandResult<String> {
    cmd_blocking!(runtime::get_temp_book_path(&book_id, &owner_id, &format))
}
