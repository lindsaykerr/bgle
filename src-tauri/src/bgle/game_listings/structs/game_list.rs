//! This module contains the GameList struct and its methods

use std::path::PathBuf;
use super::game::Game;
use serde::{Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};

/// This function creates an new GameList struct.
pub fn new() -> GameList {
    GameList {
        directory: PathBuf::new(),
        emulator: String::new(),
        games: Vec::<Game>::new(),
    }
}

/// This struct represent a list of games. It provides the emulator name and root directory.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameList {
    pub directory: PathBuf,
    pub emulator: String,
    pub games: Vec<Game>,
}

/// This enum is used to specify the type of search to be performed on the GameList struct.
pub enum SearchGames {
    Path,
    Name,
}
 

impl GameList {

    /// This method adds a game to the GameList struct.
    pub fn add_game_entry(&mut self) {
        let mut game = Game::new(self.games.len() as usize);
        self.games.push(game);
    }

    /// This method removes a game from the GameList struct.
    pub fn remove(&mut self, index: usize) {
        self.games.remove(index);
    }

    /// This method returns a reference to a game in the GameList struct.
    pub fn get(&self, index: usize) -> &Game {
        &self.games[index]
    }

    /// This search method allows the user to search specific fields of a game in the GameList struct.
    /// The search_type parameter specifies the field to be searched and the search_term parameter
    /// specifies the value to be searched for. 
    pub fn search(&self, search_type: SearchGames, search_term: &str) -> Option<usize>{
        
        
        match search_type {
            SearchGames::Path => {
                for game in &self.games {
       
                    if let Ok(found) = game.field_contains_value("path", search_term.to_string()) {
                        //println!("found: {:?}", found);
                        if found { 
                            
                            return Some(game.game_id());
                        }
                    }
                }
            },
            SearchGames::Name => {
                for game in &self.games {
                    if let Ok(found) = game.field_contains_value("name", search_term.to_string()) {
                        if found {
                            return Some(game.game_id());
                        }
                    }
                }

            },
        };
        None
    }
    pub fn len(&self) -> usize {
        self.games.len()
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_game_entry() {
        let mut game_list = new();
        game_list.add_game_entry();
        assert_eq!(game_list.games.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut game_list = new();
        game_list.add_game_entry();
        game_list.remove(0);
        assert_eq!(game_list.games.len(), 0);
    }

    #[test]
    fn test_get() {
        let mut game_list = new();
        game_list.add_game_entry();
        let game = game_list.get(0);
        assert_eq!(game.game_id(), 0);
    }

    #[test]
    fn test_search() {
        let mut game_list = new();
        game_list.add_game_entry();
        game_list.games[0].add_field("path", "test".to_string());
        let index = game_list.search(SearchGames::Path, "test");
        assert_eq!(index, Some(0));
    }

}

