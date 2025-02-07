use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageWithId {
    id: RecordId,
    uri: String,
    caption: Option<String>,
    prompt: Option<String>,
}

impl ImageWithId {
    pub fn get_id(&self) -> &RecordId {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Image {
    uri: String,
    caption: Option<String>,
    prompt: Option<String>,
}

impl Image {
    pub fn new<S: Into<String>>(uri: S) -> Self {
        Self {
            uri: uri.into(),
            caption: None,
            prompt: None,
        }
    }

    pub fn get_uri(&self) -> &String {
        &self.uri
    }

    pub fn get_caption(&self) -> &Option<String> {
        &self.caption
    }

    pub fn has_caption(&self) -> bool {
        self.caption.is_some()
    }

    pub fn set_caption(&mut self, caption: String) {
        self.caption = Some(caption);
    }

    pub fn get_prompt(&self) -> &Option<String> {
        &self.prompt
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = Some(prompt);
    }
}
