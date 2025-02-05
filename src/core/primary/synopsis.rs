use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::core::secondary::misc::Kind;
use crate::core::secondary::paragraph::Paragraph;

#[derive(Serialize, Deserialize)]
pub struct Synopsis {
    id: Ulid,
    kind: Kind,
    paragraphs: Vec<Paragraph>,
}

impl Synopsis {
    pub fn new(kind: Kind) -> Self {
        Self {
            id: Ulid::new(),
            kind,
            paragraphs: Vec::new(),
        }
    }

    pub fn get_id(&self) -> &Ulid {
        &self.id
    }

    pub fn get_kind(&self) -> &Kind {
        &self.kind
    }

    pub fn set_kind(&mut self, kind: Kind) {
        self.kind = kind;
    }

    pub fn get_paragraphs(&self) -> &Vec<Paragraph> {
        &self.paragraphs
    }

    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    pub fn remove_paragraph(&mut self, id: &Ulid) {
        self.paragraphs.retain(|paragraph| paragraph.get_id() != id);
    }
}

impl Default for Synopsis {
    fn default() -> Self {
        Self::new(Kind::Placeholder)
    }
}
