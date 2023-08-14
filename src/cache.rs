use std::collections::hash_map::Entry;
use std::collections::HashMap;
use crate::controller;
use crate::controller::models::ModifiedFile;

pub struct Cache {
    diff_strings: HashMap<String, String>
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            diff_strings: HashMap::new()
        }
    }
    
    pub fn get_diff_string(&mut self, m_file: &ModifiedFile) -> &String {
        let filename = m_file.name();
        match self.diff_strings.entry(filename) {
            Entry::Vacant(m) => m.insert(controller::get_diff_string(m_file)),
            Entry::Occupied(content) => content.into_mut()
        }
    }
}