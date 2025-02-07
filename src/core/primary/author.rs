use crate::core::secondary::misc::Gender;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Author {
    name: String,
    gender: Option<Gender>,
    email: Option<String>,
    contact: Option<String>,
    profile_img: Option<String>,
    profile_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorWithId {
    id: RecordId,
    name: String,
    gender: Option<Gender>,
    email: Option<String>,
    contact: Option<String>,
    profile_img: Option<String>,
    profile_uri: Option<String>,
}

impl AuthorWithId {
    pub fn get_id(&self) -> &RecordId {
        &self.id
    }
}

impl Author {
    pub fn new(name: String) -> Self {
        Self {
            name,
            gender: None,
            email: None,
            contact: None,
            profile_img: None,
            profile_uri: None,
        }
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
