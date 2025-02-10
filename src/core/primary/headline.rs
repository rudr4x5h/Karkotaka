use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::core::secondary::misc::Kind;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeadlineWithId {
    id: RecordId,
    content: String,
    kind: Kind,
    highlights: Vec<String>,
}

impl HeadlineWithId {
    pub fn get_id(&self) -> &RecordId {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Headline {
    content: String,
    kind: Kind,
    highlights: Vec<String>,
}

impl Headline {
    pub fn new<S: Into<String>>(content: S, kind: Kind) -> Self {
        Self {
            content: content.into(),
            kind,
            highlights: Vec::new(),
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_kind(&self) -> &Kind {
        &self.kind
    }

    pub fn get_highlights(&self) -> &Vec<String> {
        &self.highlights
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
    }
}

impl Default for Headline {
    fn default() -> Self {
        Self::new("Placeholder Headline", Kind::Placeholder)
    }
}
