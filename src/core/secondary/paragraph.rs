use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use super::misc::Kind;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParagraphWithId {
    id: RecordId,
    content: String,
    highlights: Vec<String>,
    kind: Kind,
    num_chars: usize,
    num_words: usize,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl ParagraphWithId {
    pub fn get_id(&self) -> &RecordId {
        &self.id
    }

    pub fn get_num_chars(&self) -> usize {
        self.num_chars
    }

    pub fn get_num_words(&self) -> usize {
        self.num_words
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Paragraph {
    content: String,
    highlights: Vec<String>,
    kind: Kind,
    num_chars: usize,
    num_words: usize,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Paragraph {
    pub fn new(content: String, kind: Kind) -> Self {
        Self {
            content: content.clone(),
            kind,
            highlights: Vec::new(),
            num_chars: content.clone().chars().count(),
            num_words: content.clone().split_whitespace().count(),
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }
    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.set_updated_now();
    }

    pub fn get_highlights(&self) -> &Vec<String> {
        &self.highlights
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

    pub fn get_num_chars(&self) -> usize {
        self.num_chars
    }

    pub fn get_num_words(&self) -> usize {
        self.num_words
    }

    pub fn set_updated_now(&mut self) {
        self.update_word_count();
        self.update_char_count();
        self.updated_at = Local::now();
    }

    fn update_word_count(&mut self) {
        self.num_words = self.content.split_whitespace().count();
    }

    fn update_char_count(&mut self) {
        self.num_chars = self.content.chars().count();
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new(String::new(), Kind::Placeholder)
    }
}
