use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::secondary::misc::Gender;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    id: Uuid,
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
            id: Uuid::new_v4(),
            name,
            gender: None,
            email: None,
            contact: None,
            profile_img: None,
            profile_uri: None,
        }
    }

    pub fn get_id(&self) -> &Uuid {
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
