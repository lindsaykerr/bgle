use serde::{Serialize, Deserialize};
use serde::ser::{SerializeStruct, SerializeSeq};
use std::path::Path;
use regex::Regex;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    LineText,
    Integer,
    Float,
    Bool,
    Range,
    Date,
    File,
    MultilineText,
    Alphanumeric,
}

#[derive(Debug)]
pub enum Error {
    ParseError,
    NotFound,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub editable: bool,
    pub field_type: FieldType,
    pub format_error_message: String,
}

impl Field {
    fn new(name: String, value: String, field_type: FieldType, editable: bool) -> Field {
        
        let mut value = value;
        let mut is_valid_value = false;
        let mut format_error_message = String::new(); 
        match field_type {
            FieldType::LineText => {
                if let Ok(_) = value.parse::<String>() {
                    let line_text_regex = Regex::new(r"[\n]").unwrap();
                    if !line_text_regex.is_match(&value) {
                        is_valid_value = true;
                    }
                    else {
                        format_error_message = String::from("Must contain a single line of text.");
                    }
                }
                else {
                    format_error_message = String::from("Must contain a single line of text.");
                }
            }
            FieldType::MultilineText => {
                if let Ok(_) = value.parse::<String>() {
                    is_valid_value = true;
                }
                else {
                    format_error_message = String::from("Must contain one or more lines of text.");
                }
            }
            FieldType::Float => {
                if let Ok(_) = value.parse::<f32>() {
                    is_valid_value = true;
                }
                else {
                    format_error_message = String::from("Must be a floating point number.");
                }
            },
            FieldType::Integer => {
                if let Ok(_) = value.parse::<i32>() {
                    is_valid_value = true;
                }
                else {
                    format_error_message = String::from("Must be an integer.");
                }
            },
            FieldType::Bool => {
                if let Ok(_) = value.parse::<bool>() {
                    is_valid_value = true;
                }
                else {
                    format_error_message = String::from("Must be a boolean value.");
                }
            },
            // TODO: Implement regex the following match arms
            FieldType::Range => {
                if let Ok(_) = value.parse::<String>() {
                    let range_regex = Regex::new(r"^(\d{1,3}|\d{1,3}-\d{1,3})$").unwrap();
                    if range_regex.is_match(&value) {
                        is_valid_value = true;
                    }
                    else {
                        format_error_message = String::from("Must be a one number or a range eg 1-3");
                    }
                }
                else {
                    format_error_message = String::from("Must be a one number or a range eg 1-3");
                }
            },
            FieldType::Date => {
                if let Ok(_) = value.parse::<String>() {
                    let file_date_format = Regex::new(r"^\d{8}T\d{6}$").unwrap();
                    let iso_date_format = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$").unwrap();
                    
                    if iso_date_format.is_match(&value) || file_date_format.is_match(&value) {
                        is_valid_value = true;
                    }
                    else {
                        format_error_message = String::from("Date format must be yyyymmddThhmmss or yyyy-mm-ddThh:mm:ss");
                    }
                }
                else {
                    format_error_message = String::from("Date format must be yyyymmddThhmmss or yyyy-mm-ddThh:mm:ss");
                }
            },
            FieldType::Alphanumeric => {
                if let Ok(_) = value.parse::<String>() {
                    let alphanumeric_regex = Regex::new(r"^[a-zA-Z0-9]*$").unwrap();
                    if alphanumeric_regex.is_match(&value) {
                        is_valid_value = true;
                    }
                    else {
                        format_error_message = String::from("Must be alphanumeric.");
                    }
                }
                else {
                    format_error_message = String::from("Must be alphanumeric.");
                }
            },
            FieldType::Bool => {
                if let Ok(_) = value.parse::<bool>() {
                    is_valid_value = true;
                }
                else {
                    format_error_message = String::from("Must be a boolean value.");
                }
            },
            FieldType::File => {
                if let Ok(_) = value.parse::<String>() {
                    let file_regex = Regex::new(r"^\.(/[\w _\-()]+)*(\.[A-z0-9]+)?$").unwrap();
                    if file_regex.is_match(&value) {
                        is_valid_value = true;
                    }
                    else {
                        format_error_message = String::from("Must be a file path.");
                    }
                }
                else {
                    format_error_message = String::from("Must be a file path.");
                }
            },
        }
        if !is_valid_value {
            value = String::new();
        }
        
        Field {
            name,
            value,
            editable,
            field_type,
            format_error_message,
        }
        
    }

}




#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    been_edited: bool,
    id: usize,
    pub fields: Vec<Field>,
}



impl Game {

    pub fn new(id: usize) -> Game {

        Game {
            been_edited: false,
            id,
            fields: Vec::new()
        }
    }

    pub fn add_field(&mut self, name: &str, value: String) {
        let field_type = assign_a_field_type(name);
        let is_editable = check_if_field_is_editable(name);    

        let field = Field::new(name.to_string(), value, field_type, is_editable);
        self.been_edited = true;
        self.fields.push(field);
    }

    pub fn field_contains_value(&self, field_name: &str, value: String)-> Result<bool, Error> {
        for field in &self.fields {
            if field.name.as_str() == field_name {
                if compare_strings(&field.value, &value) {
                    return Ok(true);
                } 
                else {
                    return Ok(false);
                }
            }
        }
        Err(Error::NotFound)

    }


    pub fn change_field(&mut self, field_name: &str, value: String) {
        
        
        let index_result =  self.fields.iter().position(|f| f.name.as_str() == field_name);
        
        if let Some(index) = index_result {
            let field_type = assign_a_field_type(field_name);
            let is_editable = check_if_field_is_editable(field_name);
           
            let field = Field::new(field_name.to_string(), value, field_type, is_editable);
            self.been_edited = true;
            self.fields.swap_remove(index);
            self.fields.push(field);
        }

    }

    pub fn game_id(&self) -> usize {
        self.id
    }

    pub fn has_been_edited(&mut self) -> bool {
        if self.been_edited {
            self.been_edited = false;
            true
        }
        else {
            false
        }
    }
}

fn assign_a_field_type(field_name: &str) -> FieldType {
    match field_name {
        "path" | "image" | "video" | "marquee" | "thumbnail" => FieldType::File,
        "name" | "developer" | "publisher" | "genre" | "lang" => FieldType::LineText,
        
        "desc" => FieldType::MultilineText,
        
        "lastplayed"  | "releasedate" => FieldType::Date,
    
        "players" => FieldType::Range,
        
        "cheevosId" | "playcount" => FieldType::Integer,
        "rating" => FieldType::Float,
        
        "crc32" | "md5" | "cheevosHash" => FieldType::Alphanumeric,
    
        _ => FieldType::LineText,
    }
}

fn check_if_field_is_editable(field_name: &str) -> bool {
    match field_name {
        "last_played" | "playcount" | "crc32" | "md5" | "cheevosId" | "cheevosHash" => false,
        _ => true,
    }
}


fn compare_strings(first: &String, second: &String) -> bool {
    if first.len() != second.len() {
        return false
    }
    let mut index = 0;
    let first = first.as_bytes();
    let second = second.as_bytes();

    while index < first.len() {
        if first[index] != second[index] {
            return false;
        }
        index += 1;
    }
    true
}


#[cfg(test)]
mod test {
    use super::compare_strings;

    
    #[test]
    fn test_valid_compare_strings() {
        let first = "test".to_string();
        let second = "test".to_string();
        assert_eq!(compare_strings(&first, &second), true);
    }
    #[test]
    fn test_invalid_compare_strings() {
        let first = "test".to_string();
        let second = "test2".to_string();
        assert_eq!(compare_strings(&first, &second), false);
    }
    #[test]
    fn test_compare_two_empty_strings() {
        let first = "".to_string();
        let second = "".to_string();
        assert_eq!(compare_strings(&first, &second), true);
    }
    #[test]
    fn test_compare_empty_string_with_non_empty_string() {
        let first = "".to_string();
        let second = "test".to_string();
        assert_eq!(compare_strings(&first, &second), false);
    }
    #[test]
    fn test_check_if_field_is_editable_not_editable() {
        let field_name = "last_played";
        assert_eq!(super::check_if_field_is_editable(field_name), false);
    }
    #[test]
    fn test_check_if_field_is_editable_editable() {
        let field_name = "name";
        assert_eq!(super::check_if_field_is_editable(field_name), true);
    }

    #[test]
    fn test_assign_a_field_type_string() {
        let field_name = "players";
        assert_eq!(super::assign_a_field_type(field_name), super::FieldType::Range);
    }

    #[test]
    fn test_assign_field_type_other() {
        let field_name = "test";
        assert_eq!(super::assign_a_field_type(field_name), super::FieldType::LineText);
    }

    #[test]
    fn test_new_game() {
        let id = 1;
        let game = super::Game::new(id);
        assert_eq!(game.id, id);
        assert_eq!(game.fields.len(), 0);
    }

    #[test]
    fn test_add_field_to_game() {
        let mut game = super::Game::new(1);
        let name = "name";
        let value = "test".to_string();
        game.add_field(name, value);
        assert_eq!(game.fields.len(), 1);
        assert_eq!(game.fields[0].name, name);
        assert_eq!(game.fields[0].value, "test");
    }

    #[test]
    fn test_field_contains_value_true() {
        let mut game = super::Game::new(1);
        let name = "name";
        let value = "test".to_string();
        game.add_field(name, value);
        let result = game.field_contains_value(name, "test".to_string());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_field_contains_value_false() {
        let mut game = super::Game::new(1);
        let name = "name";
        let value = "test".to_string();
        game.add_field(name, value);
        let result = game.field_contains_value(name, "test2".to_string());
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_change_field() {
        let mut game = super::Game::new(1);
        let name = "name";
        let value = "test".to_string();
        game.add_field(name, value);
        let new_value = "test2".to_string();
        game.change_field(name, new_value);
        assert_eq!(game.fields[0].value, "test2");
    }

    #[test]
    fn test_has_been_edited() {
        let mut game = super::Game::new(1);
        assert_eq!(game.has_been_edited(), false);
        game.add_field("name", "test".to_string());
        assert_eq!(game.has_been_edited(), true);
        assert_eq!(game.has_been_edited(), false);
        game.change_field("name", "test2".to_string());
        assert_eq!(game.has_been_edited(), true);
        assert_eq!(game.has_been_edited(), false); 
    }
}