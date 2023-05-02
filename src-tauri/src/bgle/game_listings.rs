pub mod structs;
pub mod display_game_list;
pub mod game_form;

use std::io::{Error, ErrorKind};
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};
use regex::Regex;
use crate::bgle::utils::file_name_to_string;

use self::structs::game_list::GameList;

use super::utils::{directory_name, directory_path_buf, TEST_DIRECTORY};
use structs::game_list;
use structs::game::Game;



/// Creates a new GameList struct from a valid directory.
/// 
pub fn new(valid_dir: &str) -> GameList {
    
    match directory_path_buf(&valid_dir.to_string()) {
        Ok(path_buf) => {
            let mut game_list = game_list::new();
            game_list.emulator = directory_name(&path_buf);  
            let mut directory = path_buf.clone();
            game_list.directory = directory;
              
            collate_games_to_list(&path_buf, &mut game_list);
            return game_list;
        }
        Err(e) => GameList { 
            directory: PathBuf::new(), 
            emulator: String::from("None"), 
            games: Vec::<Game>::new(), 
        },
    }
 
}

// Use both the game list and the directory to collate a list of games and return a GameList struct
fn collate_games_to_list(emulator_dir: &PathBuf, game_list: &mut game_list::GameList) {
   
    let directory = emulator_dir.clone();
    games_from_gamelist(&directory, game_list);
    games_found_in_dir(&directory, game_list);
}

// Collects the games found in a directory and checks if they are already in the gamelist
// to do this use compare the game path in the directory to the games path found in the gamelist
// for any games not found in the gamelist, add them to the gamelist
fn games_found_in_dir(game_list_dir: &PathBuf, game_list: &mut game_list::GameList) {

    let mut directory = game_list_dir.clone();

    for game_path in game_paths_list(&mut directory).unwrap_or(vec![]) {
        let game_path = PathBuf::from(game_path);
        let game_path = file_name_to_string(&game_path);

        // Search the game list for a game path entry if no entry is found, it is assumed that the game 
        // is not in the game list and so should be added.
        let mut path = String::from("./");
        path.push_str(&game_path);
        
        if let None = game_list.search(game_list::SearchGames::Path, &path){
        
            game_list.add_game_entry();
            let game = game_list.games.last_mut().unwrap(); 
            game.change_field("path", game_path);
        }
    }

}

// Handle the result from the dir_game_paths function and return either the list or None 
fn game_paths_list(valid_emulator_dir: &PathBuf) -> Option<Vec<String>> {

    if let Ok(paths_list) = dir_game_paths(&valid_emulator_dir) {
        if paths_list.len() > 0 {
            return Some(paths_list);
        } else {
            return None;
        }
    }
    None
}

// Returns a list of file valid game paths found within a directory
fn dir_game_paths(dir_path: &PathBuf) -> Result<Vec<String>, Error> {
    
    let mut game_paths_list = Vec::<String>::new();

    let extensions = extensions_list(&dir_path);
    // leave this function if no valid extensions were found
    if let None = extensions {
        return Err(Error::new(ErrorKind::NotFound, "No valid extensions found"));
    }
    let extensions = extensions.unwrap();
    
    for dir_entry in dir_path.read_dir()? {
        
        let path: PathBuf = dir_entry?.path(); 
        let extension = file_extension_from_path(&path);
        
        if is_valid_emulator_extension(extension, &extensions) {
            game_paths_list.push(path.to_str().unwrap().to_string());
        }
    }

    return Ok(game_paths_list);

    // Helper functions
    fn extensions_list(path_buf: &PathBuf) -> Option<Vec<String>> {
        super::utils::get_rom_extensions(&path_buf)
    }

    fn is_valid_emulator_extension(extension: Option<String>, valid_extensions: &Vec<String>) -> bool {
        if let Some(extension) = extension {
            if valid_extensions.contains(&extension) {
                return true;
            }
        }
        return false;
    }

    fn file_extension_from_path(file_path: &PathBuf) -> Option<String> {
        if let Some(extension) = file_path.extension() {
            Some(extension.to_str().unwrap().to_string())   
        }   else {
            None
        }
    }
}




// Extracts a games from the gamelist.xml file and adds them to the game list
fn games_from_gamelist(gamelist_dir: &PathBuf, game_list: &mut game_list::GameList) {

    // clone the gamelist_dir path buffer and append the gamelist.xml file name
    let mut dir_buf = gamelist_dir.clone();
    dir_buf.push("gamelist.xml");

    // read the gamelist.xml file into a string
    if let Ok(stringified_file) = std::fs::read_to_string(&dir_buf){
        
        // scrub the string, removing all new lines and tabs
        let stringified_file = stringified_file.replace("\n", "").replace("\t", "");

        // create a regex to match the contexts of the a game element 
        if let Err(e) = strip_out_games(&stringified_file, game_list) {
            println!("Error: {:?}", e);
        }
    }
}


fn strip_out_games(gamelist: &String, game_list: &mut game_list::GameList) -> Result<(), ErrorKind> {
 
    // Capture the contents of the gameList element
    let regex = Regex::new(r"<gameList>((?:.|\n)*)</gameList>").unwrap();
    let captured = regex.captures(gamelist.as_str());

    // if the contents where captured assign it to the contents variable
    if let Some(captured) = captured {
        
        let contents = captured[1].to_string();

        // find the start and end indexes of the game elements found in the contents string
        let game_start_indexes = contents.match_indices("<game>").map(|(i, _)| i).collect::<Vec<usize>>();
        let game_end_indexes = contents.match_indices("</game>").map(|(i, _)| i).collect::<Vec<usize>>();
        
        // if the number of start and end indexes are not equal, the gamelist.xml file is invalid
        if game_end_indexes.len() != game_start_indexes.len() {
            return Err(ErrorKind::InvalidData);
        }

        // iterate through the start and end indexes and extract the contents of each game element
        for i in 0..game_end_indexes.len() {
            let game_slice = contents[game_start_indexes[i]..game_end_indexes[i]].to_string();
            game_list.add_game_entry();
            // then parse out the field elements and apply them to the game struct
            parse_game_fields(game_list.games.last_mut().unwrap(), &game_slice);
        }

    }
    Ok(())
}

/*
    From the contents of a game element within the gamelist.xml file, 
    extract the field elements and apply them to a new Game struct
 */
fn parse_game_fields(game: &mut Game, entry_chunk: &String) {
    let regex = Regex::new(r"<([\w:-]+)>((?:[^<>]|\n|\w)*)</[\w:-]+>").unwrap();
    
    for tag_and_value in regex.captures_iter(entry_chunk.as_str()) {
        let tag = &tag_and_value[1];
        let value = tag_and_value[2].to_string();
        game.add_field(tag, value);  
    }
}

pub fn save(game_list: &GameList) -> Result<(), Error> {
    let xml_string = parse_gamelist_to_XML_string(game_list);
    write_to_file(&game_list.directory, &xml_string)?;
    Ok(())
}

fn write_to_file(file_path: &PathBuf, contents: &String) -> Result<(), Error> {
    println!("Attempting to write to file: {:?}", file_path);
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn parse_gamelist_to_XML_string(game_list: &GameList) -> String {
    let mut xml_string = String::new();
    xml_string.push_str("<?xml version=\"1.0\"?>\n");
    xml_string.push_str("<gameList>\n");
    for game in &game_list.games {
        xml_string.push_str("\t<game>\n");
        for field in &game.fields {
            xml_string.push_str(format!("\t\t<{}>{}</{}>\n", field.name, field.value, field.name).as_str());
        }
        xml_string.push_str("\t</game>\n");
    }
    xml_string.push_str("</gameList>\n");
    xml_string
}

#[cfg(test)]
mod test {
        
    //
    // Modify the tests to reflect the test directory on your system. Else the tests will fail.
    //
    use super::*;
    

    #[test]
    fn test_dir_game_paths() {
        let mut path = PathBuf::from(TEST_DIRECTORY);
        path.push("snes");
        let mut gamepath1 = path.clone();
        gamepath1.push("DonkeyKongClassic (Shiru).smc");
        let mut gamepath2 = path.clone();
        gamepath2.push("gameA.nes");
        let mut gamepath3 = path.clone();
        gamepath3.push("gameB.nes");
        let mut gamepath4 = path.clone();
        gamepath4.push("gameC.nes");


        let mut paths = dir_game_paths(&path).unwrap();
        
        let mut test_paths = Vec::<String>::new();
        test_paths.push(gamepath1.to_str().unwrap().to_string());
        test_paths.push(gamepath2.to_str().unwrap().to_string());
        test_paths.push(gamepath3.to_str().unwrap().to_string());
        test_paths.push(gamepath4.to_str().unwrap().to_string());

        paths.sort();
        test_paths.sort();

        assert_eq!(paths.len(), 4);

        for i in 0..paths.len() {
            assert_eq!(paths[i], test_paths[i]);
        }

    }

    #[test]
    fn test_invalid_dir_game_paths() {
        let mut path = PathBuf::from(TEST_DIRECTORY);
        let paths = dir_game_paths(&path);
        assert_eq!(paths.is_err(), true);
    }
    #[test]
    fn test_game_paths_list() {
        let mut path = PathBuf::from(TEST_DIRECTORY);
        path.push("snes");
        let paths = game_paths_list(&path).unwrap();

        assert_eq!(paths.len(), 4);
    }

    #[test]
    fn test_invalid_game_paths_list() {
        let mut path = PathBuf::from(TEST_DIRECTORY);
        let paths = game_paths_list(&path);
        assert_eq!(paths, None);
    }
    #[test]
    fn test_games_from_gamelist() {
        let mut path = PathBuf::from(TEST_DIRECTORY);
        path.push("snes");
        let mut gamelist = game_list::new();
        games_from_gamelist(&path, &mut gamelist);   
        assert_eq!(gamelist.games.len(), 3);
    }

    #[test]
    fn test_collate_games_to_list() {
        let mut path = PathBuf::from(TEST_DIRECTORY);
        path.push("snes");
        let mut game_list = game_list::new();
        collate_games_to_list(&path, &mut game_list);

        assert_eq!(game_list.games.len(), 4);
    }
 

}

