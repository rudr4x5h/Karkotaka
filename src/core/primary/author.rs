use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::core::secondary::misc::Gender;

#[derive(Serialize, Deserialize)]
pub struct Author {
    id: Ulid,
    name: String,
    gender: Option<Gender>,
    email: Option<String>,
    contact: Option<String>,
    profile_img: Option<String>,
    profile_uri: Option<String>,
}

impl Author {
    pub fn new(name: String) -> Self {
        Self {
            id: Ulid::new(),
            name,
            gender: None,
            email: None,
            contact: None,
            profile_img: None,
            profile_uri: None,
        }
    }

    pub fn get_id(&self) -> &Ulid {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn has_contact_details(&self) -> bool {
        self.email.is_some() || self.contact.is_some()
    }

}

impl Default for Author {
    fn default() -> Self {
        Self::new("Shiva".to_string())
    }
}

