//! This module contains functions used throughout multiple modules.

use std::path::{PathBuf, Path};
use regex::Regex;
use std::fs::read_to_string;
use std::io;

// Note the TEST_DIRECTORY constant is used for testing purposes only. It is not used in the main program.
pub const TEST_DIRECTORY: &'static str = "<path to test directory>/roms";

pub enum Error {
    MissingRequiredFiles,
    CannotReadDirectory,
    NoneValidPath,
}

/// Given a directory path, return a vector of strings containing acceptable file extensions
/// 
/// This function works using the _info.txt file that is usually required with an emulator directory 
/// is created. The _info.txt file contains a line that looks like this: `ROM files extensions accepted: ".smc .unif .unf .zip .7z .nes"`.
/// This function will extract the extensions from the line and return them as a vector of strings.
/// 
/// If the _info.txt file is not found, or the line is not found, the function will return an error.
/// 
/// ### Arguments
/// * `dir_buf` - A PathBuf containing a directory path to an emulator directory.
/// ### Returns
/// * `Option<Vec<String>>` - A vector of strings containing the extensions, or an error.
/// 
pub fn get_rom_extensions(dir_buf: &PathBuf) -> Option<Vec<String>>{

    // clone the PathBuf and add the _info.txt file to the end
    let mut file_path_buf = dir_buf.clone();
    file_path_buf.push("_info.txt");
    
    // Read the file contents to a string.
    match read_to_string(&file_path_buf){
        Ok(file_string) => {
            // create a vector to hold any extensions listed
            let mut extensions_vec = Vec::new();

            // create a regex to capture any listed extensions
            let regex_info = Regex::new(r#"ROM files extensions accepted:\s+"((\s?\.[A-Za-z0-9]+)*)"#).unwrap();
    
            // use the regex on file_string and then foreach captured extension add it to the extensions_vec
            for cap in regex_info.captures_iter(&file_string){
                let line = cap[1].replace(".", "");
                let extensions = line.split(" ");
                for extension in extensions {
                    extensions_vec.push(extension.to_string());
                }
            }
            
            if extensions_vec.len() == 0 {
                return None;
            }
            else {
                return Some(extensions_vec);
            }
        },
        Err(_) => None,
    }
}

/// Given a directory path, return the name of the directory
pub fn directory_name(path_buf: &PathBuf) -> String {
    file_name_to_string(&path_buf)
}

// This private function extracts a string from the PathBuf.file_name method, it is used to improve re
// readability.
pub fn file_name_to_string(path_buf: &PathBuf) -> String {
    path_buf.file_name().unwrap().to_str().unwrap().to_string()
}


/// Given a directory path as a String, return a path buffer
pub fn directory_path_buf(dir_path: &String) -> Result<PathBuf, io::Error> {
    let dir_path = Path::new(dir_path);
    if dir_path.is_dir() {
        Ok(dir_path.to_path_buf())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_valid_directory_path_buf() {
        let mut directory = String::from(TEST_DIRECTORY);
        directory.push_str("/snes");
        let mut path_buf = directory_path_buf(&directory);
        if let Ok(path_buf) = path_buf {
            assert_eq!(path_buf.exists(), true);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn test_invalid_directory_path_buf() {
        let mut directory = String::from(TEST_DIRECTORY);
        directory.push_str("/fail_emulator");
        if let Ok(path_buf) = directory_path_buf(&directory) {
            assert!(false);
        }
        else {
            assert!(true);
        }
    }

    #[test]
    fn test_valid_directory_name() {
        let mut path = String::from(TEST_DIRECTORY);
        path.push_str("/snes");
        if let Ok(path_buf) = directory_path_buf(&path) {
            let name = directory_name(&path_buf);
            assert_eq!(name, "snes");
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn test_invalid_directory_name() {
        let mut path = String::from(TEST_DIRECTORY);
        path.push_str("/hmmmm");
        if let Ok(path_buf) = directory_path_buf(&path){
            let name = directory_name(&path_buf);
        
            assert!(false);
        }
        else {
            
            assert!(true);
        }
    }

}