use serde::{Deserialize, Serialize};

use super::author::Author;

#[derive(Serialize, Deserialize)]
pub struct Domain {
    name: String,
    expert: Author,
}

impl Domain {
    pub fn new(name: String, expert: Author) -> Self {
        Self { name, expert }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_expert(&self) -> &Author {
        &self.expert
    }

    pub fn change_expert(&mut self, new_expert: Author) {
        self.expert = new_expert;
    }
}

impl Default for Domain {
    fn default() -> Self {
        Self::new("General".to_string(), Author::default())
    }
}