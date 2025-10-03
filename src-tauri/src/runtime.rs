use ridicrypt_core::prelude::*;
use ridicrypt_core::utils;
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::fs::{create_dir_all, read_dir, remove_dir_all};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static BASE_KEY: OnceLock<String> = OnceLock::new();

const TEMP_DIR_NAME: &str = "ridi-re.ridicrypt.gui";

fn get_temp_dir() -> PathBuf {
    std::env::temp_dir().join(TEMP_DIR_NAME)
}

pub fn init_temp_dir() -> Result<()> {
    let temp_dir = get_temp_dir();
    let _ = remove_dir_all(&temp_dir);
    create_dir_all(&temp_dir)?;
    Ok(())
}

pub fn cleanup_temp_dir() -> Result<()> {
    let temp_dir = get_temp_dir();
    if temp_dir.exists() {
        remove_dir_all(&temp_dir)?;
    }
    Ok(())
}

pub fn get_temp_book_path(book_id: &str, owner_id: &str, format: &str) -> Result<String> {
    let temp_dir = get_temp_dir();

    let mut hasher = DefaultHasher::new();
    book_id.hash(&mut hasher);
    owner_id.hash(&mut hasher);
    let hash = hasher.finish();

    let filename = format!("{:016x}.decrypted.{}", hash, format);
    let path = temp_dir.join(filename);

    Ok(path.to_string_lossy().replace("\\", "/"))
}

pub fn init_base_key() -> Result<()> {
    let global_key = utils::get_global_key()?;
    let settings: Value = serde_json::from_str(
        &global_key
            .decrypt_datastores(utils::get_ridi_data_path()?.join("datastores/global/Settings"))?,
    )?;
    let device_id = settings["data"]["device"]["deviceId"]
        .as_str()
        .ok_or("deviceId not found")?;

    BASE_KEY.set(device_id[..16].to_string())?;
    Ok(())
}

fn get_base_key() -> Result<&'static String> {
    BASE_KEY
        .get()
        .ok_or_else(|| "unable to get base key".into())
}

pub fn decrypt(
    key_path: impl AsRef<Path>,
    file_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
) -> Result<()> {
    let base_key = get_base_key()?;
    let key: &str = &base_key.decrypt_key(key_path)?[68..84];
    key.decrypt_zip(&file_path, &target_path)
        .or_else(|_| key.decrypt_binary(file_path, target_path))
}

pub fn get_library() -> Result<String> {
    let ridi_data_path = utils::get_ridi_data_path()?;
    let users_data = ridi_data_path.join("datastores/user");

    if !users_data.is_dir() {
        return Err("Library path not found".into());
    }

    let mut result = serde_json::Map::new();

    for entry in read_dir(&users_data)?.filter_map(|e| e.ok()) {
        let user_path = entry.path();
        if !user_path.is_dir() {
            continue;
        }

        let Some(user_id) = user_path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.strip_prefix("_").unwrap_or(s))
            .filter(|id| id.chars().all(|c| c.is_ascii_digit()))
        else {
            continue;
        };

        let Ok(book_data_key) = utils::get_user_key("DownloadBookAll", user_id) else {
            continue;
        };
        let Ok(encrypted_data) =
            book_data_key.decrypt_datastores(&user_path.join("DownloadBookAll"))
        else {
            continue;
        };
        let Ok(user_books) = serde_json::from_str::<Value>(&encrypted_data) else {
            continue;
        };
        let Some(items) = user_books.get("data").and_then(|v| v.as_array()) else {
            continue;
        };

        let meta_dir = user_path.join("BOOK_META");
        let mut user_book_map = serde_json::Map::new();

        for item in items {
            let Some(bid) = item.get("bId").and_then(|v| v.as_str()) else {
                continue;
            };

            let Ok(meta_key) = utils::get_user_key(bid, user_id) else {
                continue;
            };
            let Ok(decrypted) = meta_key.decrypt_datastores(&meta_dir.join(bid)) else {
                continue;
            };
            let Ok(mut meta_json) = serde_json::from_str::<Value>(&decrypted) else {
                continue;
            };

            if let Some(Value::Object(data_obj)) = meta_json.get_mut("data") {
                let base_path = ridi_data_path
                    .join("library")
                    .join(format!("_{}", user_id))
                    .join(bid);

                let mut storage = serde_json::Map::new();
                let base_path_str = base_path.to_string_lossy().replace("\\", "/");
                storage.insert("basePath".to_string(), Value::String(base_path_str));

                let format = data_obj
                    .get("file")
                    .and_then(|f| f.get("format"))
                    .and_then(|f| f.as_str())
                    .unwrap_or("epub");

                if base_path.is_dir() {
                    if let Ok(entries) = read_dir(&base_path) {
                        let mut filename = String::new();
                        let mut key_filename = String::new();
                        let mut create_time = String::new();

                        for entry in entries.filter_map(|e| e.ok()) {
                            let file_name = entry.file_name();
                            let name_str = file_name.to_string_lossy();

                            if name_str.contains(".decrypted.") {
                                continue;
                            }

                            if filename.is_empty()
                                && name_str.starts_with(bid)
                                && name_str.ends_with(&format!(".{}", format))
                            {
                                filename = name_str.to_string();
                                if let Ok(meta) = entry.metadata() {
                                    if let Ok(created) = meta.created() {
                                        if let Ok(datetime) =
                                            created.duration_since(std::time::UNIX_EPOCH)
                                        {
                                            create_time = datetime.as_secs().to_string();
                                        }
                                    }
                                }
                            }

                            if key_filename.is_empty()
                                && name_str.starts_with(bid)
                                && name_str.ends_with(".dat")
                            {
                                key_filename = name_str.to_string();
                            }

                            if !filename.is_empty() && !key_filename.is_empty() {
                                break;
                            }
                        }

                        storage.insert("filename".to_string(), Value::String(filename));
                        storage.insert("keyFilename".to_string(), Value::String(key_filename));
                        storage.insert("createTime".to_string(), Value::String(create_time));
                    }
                } else {
                    storage.insert("filename".to_string(), Value::String(String::new()));
                    storage.insert("keyFilename".to_string(), Value::String(String::new()));
                    storage.insert("createTime".to_string(), Value::String(String::new()));
                }

                data_obj.insert("storage".to_string(), Value::Object(storage));

                user_book_map.insert(bid.to_string(), Value::Object(data_obj.clone()));
            }
        }

        if !user_book_map.is_empty() {
            result.insert(user_id.to_string(), Value::Object(user_book_map));
        }
    }

    Ok(serde_json::to_string(&Value::Object(result))?)
}
