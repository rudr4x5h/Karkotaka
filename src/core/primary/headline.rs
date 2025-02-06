use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::secondary::misc::Kind;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Headline {
    id: Uuid,
    content: String,
    kind: Kind,
}

impl Headline {
    pub fn new<S: Into<String>>(content: S, kind: Kind) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: content.into(),
            kind,
        }
    }

    pub fn get_id(&self) -> &Uuid {
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
        Self::new("Placeholder Headline", Kind::Placeholder)
    }
}
