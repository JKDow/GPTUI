use std::path::PathBuf;
#[allow(unused_imports)]
use app_dirs2::{app_root, AppInfo, AppDataType}; 
#[allow(dead_code)]
const APP_INFO: AppInfo = AppInfo {name: "gptui", author: "jkdow"};

#[cfg(not(debug_assertions))]
pub fn get_config_root() -> PathBuf {
    // return config path using app_dirs2
    app_root(AppDataType::UserConfig, &APP_INFO).unwrap()
}

#[cfg(debug_assertions)]
pub fn get_config_root() -> PathBuf {
    // The PathBuf::from() function creates a new PathBuf from a string slice.
    std::path::PathBuf::from("./files/config/")
}

#[cfg(not(debug_assertions))]
pub fn get_data_root() -> PathBuf {
    app_root(AppDataType::UserData, &APP_INFO).unwrap()
}

#[cfg(debug_assertions)]
pub fn get_data_root() -> PathBuf {
    PathBuf::from("./files/data")
}

#[cfg(not(debug_assertions))]
pub fn get_cache_root() -> PathBuf {
    app_root(AppDataType::UserData, &APP_INFO).unwrap()
}

#[cfg(debug_assertions)]
pub fn get_cache_root() -> PathBuf {
    PathBuf::from("./files/cache")
}