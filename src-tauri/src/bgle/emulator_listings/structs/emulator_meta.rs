/*
    This module contains the functions for supporting the assignment of Emulator metadata.
 */

use super::super::utils::{directory_name, directory_path_buf, get_rom_extensions};
use std::fs::{read_to_string};
use std::path::PathBuf;
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde::ser::SerializeStruct;
use super::super::utils::TEST_DIRECTORY;




/**
 * Creates a new EmulatorMeta struct based on the on a valid emulator directory path.
 */
pub fn new(directory: &str) -> EmulatorMeta {
    match directory_path_buf(&directory.to_string()) {
        Ok(dir_path_buf) => {
            
            let name            = directory_name(&dir_path_buf);
            let extensions      = get_rom_extensions(&dir_path_buf).unwrap();
            let dir_game_count  = count_games_in_the_directory(&dir_path_buf, &extensions);
            let gamefile_meta   = gamefile_game_count_and_field(&dir_path_buf).unwrap_or((0,0));
            
            let file_game_count  = gamefile_meta.0;
            let total_elements   = gamefile_meta.1;
            
            return EmulatorMeta {
                name,
                directory: dir_path_buf,
                gamefile_elements: total_elements,
                game_count: if file_game_count < dir_game_count {dir_game_count} else {file_game_count},
                rom_extensions: extensions,
            }
        },
        Err(_) => panic!("Error: Could not create EmulatorMeta struct, likely due to an invalid directory path.")
    }

}


// Counts the game files within the directory.
fn count_games_in_the_directory(dir_path: &PathBuf, extensions: &Vec<String>) -> u32 {
    
    let mut game_count = 0;
    
    for entry in dir_path.read_dir().unwrap() {
        
        let entry = entry.unwrap();
        let file_path = entry.path();

        if let Some(extension) = file_path.extension() {   
            
            let file_extension = extension.to_str().unwrap();
            
            if extensions.contains(&file_extension.to_string()) {
                game_count += 1;
            }
        }
    }
    
    game_count
}

// Counts the games listed with the gamefile and the fields that are filled in.
fn gamefile_game_count_and_field(dir_path: &PathBuf) -> Option<(u32, u32)> {
    let mut game_count = 0;
    let mut field_count = 0;
    
    let mut file_path_buf = dir_path.clone();
    file_path_buf.push("gamelist.xml");

    if let Ok(file_string) = read_to_string(&file_path_buf){

        let regex_game  = Regex::new(r"<game>").unwrap();
        let regex_field = Regex::new(r"<\s?\w+\s?>(.*)</\s?\w+\s?>").unwrap();

        for _cap in regex_game.captures_iter(&file_string){
            game_count += 1;
        }

        if game_count > 0 {
            for _cap in regex_field.captures_iter(&file_string){
                field_count += 1;
            }
        }

        Some((game_count, field_count))
    
    } else {
        None
    }
}

#[derive(Debug, Deserialize)]
pub struct EmulatorMeta {
    pub name: String,
    directory : std::path::PathBuf,
    gamefile_elements: u32,
    game_count: u32,
    rom_extensions: Vec<String>,
}

impl EmulatorMeta {
    /* 
        Provides the percentage of metadata assigned to the games 
        in the directory.
        */
    pub fn complete_percent(&self) -> f32 {

        const NO_GAMELIST_GAME_PROPS: u32 = 14;
        let max_fields = self.game_count * NO_GAMELIST_GAME_PROPS;
        
        if max_fields != 0 {
            (self.gamefile_elements as f32 / max_fields as f32) * 100.0
        }
        else {
            0.0
        }
        
    }

    /* 
        Provides the actual number of games in the directory
        based on the number of files with the correct extension.
    */
    pub fn game_count(&self) -> u32 {
        self.game_count
    }

    pub fn directory_path(&self) -> String {
        self.directory.to_str().unwrap().to_string()
    }



}
impl Serialize for EmulatorMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut s = serializer.serialize_struct("EmulatorMeta", 5)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("directory", &self.directory_path())?;
        s.serialize_field("gamefile_elements", &self.gamefile_elements)?;
        s.serialize_field("game_count", &self.game_count)?;
        s.serialize_field("rom_extensions", &self.rom_extensions)?;
        s.serialize_field("complete_percent", &self.complete_percent())?;
        s.end()
    }
}


#[cfg(test)]
mod test {
    
    //
    // Modify the tests to reflect the test directory on your system. Else the tests will fail.
    //
    use super::*;
    #[test]
    fn test_get_rom_extensions() {
        let mut directory = String::from(TEST_DIRECTORY);
        directory.push_str("/nes");
        let dir_path_buf = directory_path_buf(&directory).unwrap();
        match get_rom_extensions(&dir_path_buf) {
            Some(extensions) => {
                let extensions_str = extensions.join(" ");
                let extensions_test = vec!["nes".to_string(), "unif".to_string(), "unf".to_string(), "zip".to_string(), "7z".to_string()].join(" ");
                assert_eq!(
                    extensions_str,
                    extensions_test
                );
            },
            None => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_count_games_in_the_directory() {
        let mut directory = String::from(TEST_DIRECTORY);
        directory.push_str("/nes");
        let mut dir_path_buf = directory_path_buf(&directory).unwrap();
        match get_rom_extensions(&dir_path_buf) {
            Some(extensions) => {
                let game_count = count_games_in_the_directory(&dir_path_buf, &extensions);
                assert_eq!(game_count, 1);
            },
            None => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_gamefile_game_count_and_field() {
        let mut directory = String::from(TEST_DIRECTORY);
        directory.push_str("/nes");
        let mut dir_path_buf = directory_path_buf(&directory).unwrap();
        match gamefile_game_count_and_field(&dir_path_buf) {
            Some((game_count, field_count)) => {
                assert_eq!(game_count, 1);
                assert_eq!(field_count, 19);
            },
            None => {
                assert!(false);
            }
        }
    }

}

    