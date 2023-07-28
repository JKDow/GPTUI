use std::{path::PathBuf, fs::DirEntry, time::SystemTime, io::{Read, Write}};

use crate::paths::get_cache_root;


pub fn save_chat(path: PathBuf) {
    //from chat logs directory read the last chat log
    let mut newest: Option<DirEntry> = None;
    let mut newest_time: Option<SystemTime> = None;
    let mut logs_dir = get_cache_root();
    logs_dir.push("chats");
    for entry in std::fs::read_dir(logs_dir).unwrap() {
        let entry = entry.unwrap();
        let data = entry.metadata().unwrap(); 
        if !data.is_file() {
            continue;
        }
        let created: SystemTime = data.created().unwrap();
        if newest.is_none() || (data.created().unwrap() > newest_time.unwrap()) {
            newest = Some(entry);
            newest_time = Some(created);
        }
    }
    match newest {
        Some(entry) => {
            let mut file = std::fs::File::open(entry.path()).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut file = std::fs::File::create(path).unwrap();
            file.write_all(contents.as_bytes()).unwrap();
        },
        None => {
            println!("No chat logs found");
        }
    }
}

/// ## Overview
/// Clears all chats in chat logs 
/// 
/// ## WARNING
/// This will delete all files in the configured directory
/// 
/// Operation is **not** reversable 
pub fn clear_chat_logs() -> u32 {
    // Clear all chat logs at path 
    let mut path = get_cache_root();
    path.push("chats");
    let mut num = 0; 
    for entry in std::fs::read_dir(path).unwrap() {
        num += 1;
        let entry = entry.unwrap();
        let data = entry.metadata().unwrap(); 
        if !data.is_file() {
            continue;
        }
        std::fs::remove_file(entry.path()).unwrap();
    }
    return num;
}