
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bgle;
use bgle::game_listings;
use bgle::game_listings::structs::game::{Game, self};
use bgle::game_listings::structs::game_list::GameList;
use bgle::emulator_listings::list;
use bgle::emulator_listings::structs::emulator_list::EmulatorList;

use std::path::PathBuf;



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[tauri::command(rename_all = "snake_case")]
fn save_game_list(game_list: GameList) -> Result<String, String> {
    let mut game_list = game_list;
    let mut game_directory = PathBuf::from(game_list.directory.to_str().unwrap());
    game_directory.push("gamelist.xml");
    game_list.directory = game_directory;
  

    if game_list.games.len() > 0 {
        match game_listings::save(&game_list) {
            Ok(_) => Ok(String::from("Game list saved")),
            Err(e) => Err(e.to_string()),
        }
       
    }
    else {
        Err(String::from("No games found"))
    }


}


#[tauri::command]
fn get_emulator_list(path: &str) -> Result<EmulatorList, String> {
    
    let mut buf_path = PathBuf::new();
    buf_path.push(path);
    if buf_path.exists() { 
        if let Some(emulators) = list(path) {
            Ok(emulators)
        }
        else {
            Err(String::from("No emulators found"))
        }
    } 
    else {
        Err(String::from(path))
    }   
}

#[tauri::command(rename_all = "snake_case")]
fn get_game_list(valid_dir: &str) ->Result<GameList, String> {
    let games_list: GameList = game_listings::new(&valid_dir);
    
    if games_list.len() > 0 {
        Ok(games_list)
    }
    else {
        Err(String::from("No games found"))
    }
}

fn main() {
    tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![greet, get_emulator_list, get_game_list, save_game_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
