use std::path::PathBuf;

use chrono::NaiveDateTime;

use crate::{cli::input::{ChatCmd, ChatSubCmd}, config::MainConfig, open_ai::chat::OaiChat, paths::get_cache_root};

pub async fn run_chat_session(config: MainConfig, chat_info: ChatCmd) {
    match chat_info.chat_sub_cmd{
        Some(sub_cmd) => {
            match sub_cmd {
                ChatSubCmd::Start(start_cmd) => {
                    let model = match start_cmd.model {
                        Some(model) => model,
                        None => config.gptui.default_model,
                    };
                    let mut chat = OaiChat::new(
                        model,
                        true,
                        config.gptui.api_key,
                        get_cache_root(),
                    );
                    chat.basic_loop().await;
                }
                ChatSubCmd::Save(save_cmd) => {
                    let mut path = get_cache_root(); 
                    let to_save = match save_cmd.name {
                        Some(name) => {
                            path.push(name);
                            path
                        }
                        None => {
                            get_most_recent(path).unwrap()
                        }
                    };
                    // copy from to_save path to save_cmd.path
                    std::fs::copy(to_save, save_cmd.path).unwrap();
                }
                ChatSubCmd::Load(load_cmd) => {

                }
                ChatSubCmd::Clear(clear_cmd) => {

                }
                ChatSubCmd::Show(show_cmd) => {

                }
                ChatSubCmd::List(list_cmd) => {
                    
                }
            }
        }
        None => {
            let mut chat = OaiChat::new(
                config.gptui.default_model,
                true,
                config.gptui.api_key,
                get_cache_root(),
            );
            chat.basic_loop().await;
        }
    }
}

pub fn get_most_recent(path: PathBuf) -> Option<PathBuf> {
    // read all files at path 
    let mut paths: Vec<_> = std::fs::read_dir(path).unwrap()
        .filter_map(|res| {
            res.ok()
                .and_then(|entry| {
                    let path = entry.path();
                    let filename = path.file_stem()?.to_str()?;
                    let parts: Vec<&str> = filename.split('_').collect();
                    if parts.len() != 3 {
                        return None;
                    }

                    let timestamp = format!("{} {}", parts[1], parts[2]);
                    let datetime = NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%d %H-%M-%S").unwrap();
                    Some((datetime, path))
                })
        })
        .collect();

    paths.sort_by_key(|&(datetime, _)| datetime);
    println!("{:?}", paths);

    Some(paths.pop().unwrap().1)
}