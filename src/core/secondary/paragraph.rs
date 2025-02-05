use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::misc::Kind;

#[derive(Serialize, Deserialize, Clone)]
pub struct Paragraph {
    id: Ulid,
    content: String,
    kind: Kind,
    num_chars: u8,
    num_words: u8,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Paragraph {
    pub fn new(content: String, kind: Kind) -> Self {
        Self {
            id: Ulid::new(),
            content: content.clone(),
            kind,
            num_chars: content.clone().chars().count() as u8,
            num_words: content.clone().split_whitespace().count() as u8,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }

    pub fn get_id(&self) -> &Ulid {
        &self.id
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.set_updated_now();
    }

    pub fn get_kind(&self) -> &Kind {
        &self.kind
    }

    pub fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
        self.set_updated_now();
    }

    pub fn get_created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    pub fn get_updated_at(&self) -> &DateTime<Local> {
        &self.updated_at
    }

    pub fn get_num_chars(&self) -> u8 {
        self.num_chars
    }

    pub fn get_num_words(&self) -> u8 {
        self.num_words
    }

    pub fn set_updated_now(&mut self) {
        self.update_word_count();
        self.update_char_count();
        self.updated_at = Local::now();
    }

    fn update_word_count(&mut self) {
        self.num_words = self.content.split_whitespace().count() as u8;
    }

    fn update_char_count(&mut self) {
        self.num_chars = self.content.chars().count() as u8;
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new(String::new(), Kind::Placeholder)
    }
}
