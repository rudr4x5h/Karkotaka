use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::core::secondary::misc::Kind;

#[derive(Serialize, Deserialize)]
pub struct Headline {
    id: Ulid,
    content: String,
    kind: Kind,
}

impl Headline {
    pub fn new(content: String, kind: Kind) -> Self {
        Self { id: Ulid::new(), content, kind }
    }

    pub fn get_id(&self) -> &Ulid {
        &self.id
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_kind(&self) -> &Kind {
        &self.kind
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
        Self::new("Placeholder Headline".to_string(), Kind::OG)
    }
}