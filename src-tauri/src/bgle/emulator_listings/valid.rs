use std::path::PathBuf;
use std::fs::ReadDir;
use super::super::utils::Error;



/// Searches a directory for valid directories and returns a vector of valid directory paths.
/// A valid directory path contains a gamelist.xml and _info.txt file

pub fn valid_directories_list(path: &str) -> Result<Vec<String>, Error> {

    // test if the system path is valid
    let valid_path = valid_path(path)?;
    let dir_entries = directory_reader(valid_path)?; 

    // the vector which will hold valid directory paths
    let mut valid_paths = Vec::<String>::new();     

    // for every entery in the directory...    
    for entry in dir_entries {
        if let Ok(entry) = entry {
            // ...check if the entry is a directory.
            if entry.path().is_dir() {
                // if it is a valid directory, add it to the vector
                if let Ok(valid_path) = valid_directory_entry(&mut entry.path()) {
                    valid_paths.push(valid_path);
                }
            }
        }
    }
    
    Ok(valid_paths)

}

// Private function which determines if a path is valid and returns a PathBuf
fn valid_path(path: &str) -> Result<PathBuf, Error> {
    let path = PathBuf::from(path);
    if path.exists() {
        Ok(path)
    } else {
        Err(Error::NoneValidPath)
    }
}

// Provides the means to traverse a directory using a ReadDir iterator
fn directory_reader(valid_path: PathBuf) -> Result<ReadDir, Error> {
    let result = valid_path.read_dir();
    match result {
        Ok(dir_entries) => Ok(dir_entries),
        Err(_) => Err(Error::CannotReadDirectory),
    }
}

// Determines if a directory contains the required files
fn valid_directory_entry(dir_path: &PathBuf) -> Result<String, Error> {

    let mut path_buf = dir_path.clone();

    if check_file_exists(&mut path_buf, "gamelist.xml")
    && check_file_exists(&mut path_buf, "_info.txt") {
        Ok(dir_path.to_str().unwrap().to_string())
    }
    else {
        Err(Error::MissingRequiredFiles)
    }
}

// Checks if given a file name, that a file exists in a directory
fn check_file_exists(path: &mut PathBuf, file_name: &str) -> bool {
    
    let mut file_exists = false;
    
    path.push(file_name);
    if path.exists() {
        file_exists = true;
    }
    path.pop();

    file_exists
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path() {
        let path = valid_path("/tmp");
        assert!(path.is_ok());
    }
    #[test]
    fn test_invalid_path() {
        let path = valid_path("/tmp/does-not-exist");
        assert!(path.is_err());
    }
}
