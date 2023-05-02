pub mod valid;
pub mod structs;
use super::utils;
use structs::emulator_list;

// create a new emulator_list struct
pub fn list(a_path: &str) -> Option<emulator_list::EmulatorList> {

    match valid::valid_directories_list(&a_path) {
        Ok(directories_list) => {
            
            let mut emulator_list = emulator_list::new();
            for directory in directories_list {
                let emulator_meta = structs::emulator_meta::new(&directory);
                emulator_list.add(emulator_meta);
            }
            return Some(emulator_list);
        
        },
        Err(e) => {
            match e {
                utils::Error::NoneValidPath => println!("{}, is not a valid path", &a_path),
                utils::Error::CannotReadDirectory => println!("Cannot read directory {}", &a_path),
                utils::Error::MissingRequiredFiles => println!("Emulator path does not seem to be valid {}", &a_path),
            }
            return None;
        },
    }    

}
