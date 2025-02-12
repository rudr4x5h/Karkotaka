use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::core::secondary::misc::Kind;
use crate::core::secondary::paragraph::{Paragraph, ParagraphWithId};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SynopsisWithId {
    id: RecordId,
    kind: Kind,
    paragraphs: Vec<ParagraphWithId>,
}

impl SynopsisWithId {
    pub fn get_id(&self) -> &RecordId {
        &self.id
    }

    pub fn remove_paragraph(&mut self, id: &RecordId) {
        self.paragraphs.retain(|paragraph| paragraph.get_id() != id);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Synopsis {
    kind: Kind,
    paragraphs: Vec<Paragraph>,
}

impl Synopsis {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            paragraphs: Vec::new(),
        }
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

    pub fn exists_generated_para(&self) -> bool {
        let generate_para: Vec<&Paragraph> = self
            .get_paragraphs()
            .iter()
            .filter(|p| *p.get_kind() == Kind::AI)
            .collect();
        generate_para.len() > 1
    }
}

impl Default for Synopsis {
    fn default() -> Self {
        Self::new(Kind::Placeholder)
    }
}
