use serde::{Deserialize, Serialize};

use super::author::Author;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Domain {
    name: DomainName,
    expert: Author,
}

impl Domain {
    pub fn new(name: DomainName, expert: Author) -> Self {
        Self { name, expert }
    }

    pub fn get_name(&self) -> &DomainName {
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
        Self::new(DomainName::Undefined, Author::default())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DomainName {
    Agriculture,
    Sustainability,
    Consciousness,
    Cybersecurity,
    Undefined,
}
