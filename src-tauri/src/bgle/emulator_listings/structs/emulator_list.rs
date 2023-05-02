use super::emulator_meta::EmulatorMeta;
use serde::{Serialize, Deserialize};
use serde::ser::SerializeStruct;
/*
    This module contains the structs required for listing emulators and 
    their respective metadata.
 */
pub fn new() -> EmulatorList {
    EmulatorList {
        emulators: Vec::<EmulatorMeta>::new(),
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EmulatorList {
    pub emulators: Vec<EmulatorMeta>,
}

impl EmulatorList {
    pub fn add(&mut self, emulator: EmulatorMeta) {
        self.emulators.push(emulator);
    }

    pub fn remove(&mut self, index: usize) {
        self.emulators.remove(index);
    }

    pub fn get(&self, index: usize) -> &EmulatorMeta {
        &self.emulators[index]
    }
}

